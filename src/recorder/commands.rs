/*
vkCmdBeginConditionalRenderingEXT(3)
vkCmdBeginDebugUtilsLabelEXT(3)
vkCmdBeginQuery(3)
vkCmdBeginQueryIndexedEXT(3)
vkCmdBeginRenderPass(3)
vkCmdBeginRenderPass2(3)
vkCmdBeginTransformFeedbackEXT(3)
vkCmdBindDescriptorSets(3)
vkCmdBindPipeline(3)
vkCmdBindShadingRateImageNV(3)
vkCmdBlitImage(3)
vkCmdBuildAccelerationStructureNV(3)
vkCmdClearAttachments(3)
vkCmdClearColorImage(3)
vkCmdClearDepthStencilImage(3)
vkCmdCopyAccelerationStructureNV(3)
vkCmdCopyBufferToImage(3)
vkCmdCopyImage(3)
vkCmdCopyQueryPoolResults(3)
vkCmdDebugMarkerBeginEXT(3)
vkCmdDebugMarkerEndEXT(3)
vkCmdDebugMarkerInsertEXT(3)
vkCmdDispatch(3)
vkCmdDispatchBase(3)
vkCmdDispatchIndirect(3)
vkCmdDraw(3)
vkCmdDrawIndexed(3)
vkCmdDrawIndexedIndirect(3)
vkCmdDrawIndexedIndirectCount(3)
vkCmdDrawIndirect(3)
vkCmdDrawIndirectByteCountEXT(3)
vkCmdDrawIndirectCount(3)
vkCmdDrawMeshTasksIndirectCountNV(3)
vkCmdDrawMeshTasksIndirectNV(3)
vkCmdDrawMeshTasksNV(3)
vkCmdEndConditionalRenderingEXT(3)
vkCmdEndDebugUtilsLabelEXT(3)
vkCmdEndQuery(3)
vkCmdEndQueryIndexedEXT(3)
vkCmdEndRenderPass(3)
vkCmdEndRenderPass2(3)
vkCmdEndTransformFeedbackEXT(3)
vkCmdExecuteCommands(3)
vkCmdInsertDebugUtilsLabelEXT(3)
vkCmdNextSubpass(3)
vkCmdNextSubpass2(3)
vkCmdPipelineBarrier(3)
vkCmdProcessCommandsNVX(3)
vkCmdPushConstants(3)
vkCmdPushDescriptorSetKHR(3)
vkCmdPushDescriptorSetWithTemplateKHR(3)
vkCmdReserveSpaceForCommandsNVX(3)
vkCmdResetEvent(3)
vkCmdResetQueryPool(3)
vkCmdResolveImage(3)
vkCmdSetBlendConstants(3)
vkCmdSetCheckpointNV(3)
vkCmdSetCoarseSampleOrderNV(3)
vkCmdSetDepthBias(3)
vkCmdSetDepthBounds(3)
vkCmdSetDeviceMask(3)
vkCmdSetDiscardRectangleEXT(3)
vkCmdSetEvent(3)
vkCmdSetExclusiveScissorNV(3)
vkCmdSetLineStippleEXT(3)
vkCmdSetLineWidth(3)
vkCmdSetPerformanceMarkerINTEL(3)
vkCmdSetPerformanceOverrideINTEL(3)
vkCmdSetPerformanceStreamMarkerINTEL(3)
vkCmdSetSampleLocationsEXT(3)
vkCmdSetScissor(3)
vkCmdSetStencilCompareMask(3)
vkCmdSetStencilReference(3)
vkCmdSetStencilWriteMask(3)
vkCmdSetViewport(3)
vkCmdSetViewportShadingRatePaletteNV(3)
vkCmdSetViewportWScalingNV(3)
vkCmdTraceRaysNV(3)
vkCmdWaitEvents(3)
vkCmdWriteAccelerationStructuresPropertiesNV(3)
vkCmdWriteBufferMarkerAMD(3)
vkCmdWriteTimestamp(3)
*/

use ash::vk;

// Any type of command that can be applied
pub(crate) enum Command {
    // Any type of command that gets applied to buffers
    Buffer(super::BufferCommand),
}

// Records the commands in the actual command buffer
pub(crate) trait Finish {
    // Record the commands into the given command buffer
    unsafe fn finish(self, device: &ash::Device, buffer: vk::CommandBuffer);
}

impl Finish for Command {
    unsafe fn finish(self, device: &ash::Device, buffer: vk::CommandBuffer) {
        match self {
            Command::Buffer(x) => x.finish(device, buffer),
        }
    }
}

