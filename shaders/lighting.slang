#include <other.slang>

static const float3 sun = normalize(float3(1, 1, 1));

float3 sky(float3 dir) {
    float3 sky1 = pow(float3(52, 186, 235) / 255.0, 2.2);
    float3 sky2 = pow(float3(20, 45, 105) / 255.0, 2.2);
    float3 color = lerp(sky1, sky2, dir.y);
    return color + pow(max(dot(dir, sun), 0), 100) * 10;
}

float3 light(Fetcher fetcher, uint3 id, float3 world, float3 uv, float3 normal, float ao) {
    uint3 pixels = (uint3)(floor(uv * 8 + 0.001));

    if (hash13(id) > 0.5) {
        pixels.x = 7 - pixels.x;
    }

    if (hash13(id + 12.12321) > 0.5) {
        pixels.z = 7 - pixels.z;
    }

    normal = normalize(normal + (hash33(pixels * float3(4.5984, 43.2323, -0.1212)) - 0.5) * 0.5);
    float mixer = hash13(pixels * float3(123.321, 21.322, -32.321)) * 0.2 + 0.8;

    float3 grass = pow(float3(17, 33, 19) / 255, 2.2);
    float3 dirt = pow(float3(41, 37, 30) / 255, 2.2);


    float ndotl = 2 * max(dot(normal, sun), 0);

    bool top_face = normal.y > 0.5;

    bool top_pixels = pixels.y > 6 && !fetcher.fetch(id + uint3(0, 1, 0)).active;

    float3 color = ((top_pixels || top_pixels) ? grass : dirt);

    float3 diffuse = color * mixer;
    float3 ambient = sky(normal);

    return (1 * diffuse * ndotl + diffuse * 1.5) * (ao * 0.5 + 0.5);
}


// Narkowicz 2015, "ACES Filmic Tone Mapping Curve"
float3 aces(float3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

struct ao_solver {
    Fetcher fetcher;
    uint3 pos;
    float3 uv;
    int face;
    float3 sign;

    bool check(int3 offset, int target_face) {
        if (sign[target_face] < 0) {
            offset[target_face] = -offset[target_face];
        }

        return face == target_face && fetcher.fetch(pos + offset).active;
    }

    float check_axis(int i, int a, int b) {
        bool4 c = false;

        static const int2[] array = {
            int2(-1, -1),
            int2(-1, 1),
            int2(1, -1),
            int2(1, 1),
        };

        static const int2[] array2 = {
            int2(-1, 0),
            int2(1, 0),
            int2(0, -1),
            int2(0, 1),
        };

        static const int[] corner_from_side_lookup = {
            0, 1,
            2, 3,
            0, 2,
            1, 3
        };


        for (int k = 0; k < 4; k++) {
            int3 corner = 0;
            corner[i] = -1;
            corner[a] = array[k].x;
            corner[b] = array[k].y;
            c[k] = check(corner, i);
        }

        for (int s = 0; s < 4; s++) {
            int3 side = 0;
            side[i] = -1;
            side[a] = array2[s].x;
            side[b] = array2[s].y;

            int id1 = corner_from_side_lookup[s * 2];
            int id2 = corner_from_side_lookup[s * 2 + 1];

            if (check(side, i)) {
                c[id1] = true;
                c[id2] = true;
            }
        }

        float4 selected = select(c, 1, 0);
        float interpolated = 1 - lerp(lerp(selected.x, selected.y, uv[b]), lerp(selected.z, selected.w, uv[b]), uv[a]);
        return interpolated;
    }

    float ao() {
        float x = check_axis(0, 1, 2);
        float z = check_axis(2, 1, 0);
        float y = check_axis(1, 0, 2);
        return x * y * z;
    }
}