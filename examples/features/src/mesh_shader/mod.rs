enum RenderPath {
    Mesh(wgpu::RenderPipeline),
    Vertex(wgpu::RenderPipeline),
}

fn supports_mesh_shaders(features: wgpu::Features) -> bool {
    features
        .contains(wgpu::Features::EXPERIMENTAL_MESH_SHADER | wgpu::Features::PASSTHROUGH_SHADERS)
}

pub struct Example {
    path: RenderPath,
}
impl crate::framework::Example for Example {
    fn init(
        config: &wgpu::SurfaceConfiguration,
        _adapter: &wgpu::Adapter,
        device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) -> Self {
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let primitive = wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        };

        let path = if supports_mesh_shaders(device.features()) {
            let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
            RenderPath::Mesh(device.create_mesh_pipeline(&wgpu::MeshPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                task: Some(wgpu::TaskState {
                    module: &shader,
                    entry_point: Some("ts_main"),
                    compilation_options: Default::default(),
                }),
                mesh: wgpu::MeshState {
                    module: &shader,
                    entry_point: Some("ms_main"),
                    compilation_options: Default::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(config.view_formats[0].into())],
                }),
                primitive,
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            }))
        } else {
            let shader = device.create_shader_module(wgpu::include_wgsl!("fallback.wgsl"));
            RenderPath::Vertex(
                device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("mesh shader fallback"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: Default::default(),
                        buffers: &[],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: Default::default(),
                        targets: &[Some(config.view_formats[0].into())],
                    }),
                    primitive,
                    depth_stencil: None,
                    multisample: Default::default(),
                    multiview_mask: None,
                    cache: None,
                }),
            )
        };
        Self { path }
    }
    fn render(&mut self, view: &wgpu::TextureView, device: &wgpu::Device, queue: &wgpu::Queue) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            rpass.push_debug_group("Prepare data for draw.");
            let pipeline = match &self.path {
                RenderPath::Mesh(pipeline) | RenderPath::Vertex(pipeline) => pipeline,
            };
            rpass.set_pipeline(pipeline);
            rpass.pop_debug_group();
            rpass.insert_debug_marker("Draw!");
            match &self.path {
                RenderPath::Mesh(_) => rpass.draw_mesh_tasks(1, 1, 1),
                RenderPath::Vertex(_) => rpass.draw(0..3, 0..1),
            }
        }
        queue.submit(Some(encoder.finish()));
    }
    fn required_downlevel_capabilities() -> wgpu::DownlevelCapabilities {
        Default::default()
    }
    fn optional_features() -> wgpu::Features {
        wgpu::Features::EXPERIMENTAL_MESH_SHADER | wgpu::Features::PASSTHROUGH_SHADERS
    }
    fn required_limits() -> wgpu::Limits {
        wgpu::Limits::defaults().using_recommended_minimum_mesh_shader_values()
    }
    fn required_limits_for_adapter(adapter: &wgpu::Adapter) -> wgpu::Limits {
        if supports_mesh_shaders(adapter.features()) {
            Self::required_limits()
        } else {
            wgpu::Limits::downlevel_webgl2_defaults()
        }
    }
    fn resize(
        &mut self,
        _config: &wgpu::SurfaceConfiguration,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
        // empty
    }
    fn update(&mut self, _event: winit::event::WindowEvent) {
        // empty
    }
}

pub fn main() {
    crate::framework::run::<Example>("mesh_shader");
}

#[cfg(test)]
#[wgpu_test::apply(wgpu_test::gpu_test!)]
pub static TEST: crate::framework::ExampleTestParams = crate::framework::ExampleTestParams {
    name: "mesh_shader",
    image_path: "/examples/features/src/mesh_shader/screenshot.png",
    width: 1024,
    height: 768,
    optional_features: wgpu::Features::default(),
    base_test_parameters: wgpu_test::TestParameters::default()
        .features(wgpu::Features::EXPERIMENTAL_MESH_SHADER | wgpu::Features::PASSTHROUGH_SHADERS)
        .instance_flags(wgpu::InstanceFlags::advanced_debugging())
        .limits(wgpu::Limits::defaults().using_recommended_minimum_mesh_shader_values())
        .skip(wgpu_test::FailureCase {
            backends: None,
            // Skip Mesa because LLVMPIPE has what is believed to be a driver bug
            vendor: Some(0x10005),
            adapter: None,
            driver: None,
            reasons: vec![],
            behavior: wgpu_test::FailureBehavior::Ignore,
        }),
    comparisons: &[wgpu_test::ComparisonType::Mean(0.005)],
    _phantom: std::marker::PhantomData::<Example>,
};
