use winit::{event::{ElementState, MouseButton, WindowEvent}, window::Window};
use rand::{Rng, thread_rng};

pub struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub size: winit::dpi::PhysicalSize<u32>,
    clear_colour: wgpu::Color
}

impl State {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface)
        }).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            features: wgpu::Features::default(),
            limits: wgpu::Limits::default(),
            shader_validation: true
        }, None).await.unwrap();
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let clear_colour = wgpu::Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0
        };

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            clear_colour
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::MouseInput { button: MouseButton::Left, state: ElementState::Pressed, .. } => {
                self.clear_colour = self.random_colour();
                return true;
            }
            _ => {}
        }

        false
    }

    pub fn update(&mut self) {

    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.clear_colour),
                            store: true,
                        }
                    }
                ],
                depth_stencil_attachment: None
            });
        }

        self.queue.submit(std::iter::once(encoder.finish()));

        Ok(())
    }

    fn random_colour(&mut self) -> wgpu::Color {
        let mut rng = rand::thread_rng();

        wgpu::Color {
            r: rng.gen(),
            g: rng.gen(),
            b: rng.gen(),
            a: 1.0,
        }
    }
}