// Adapted from https://github.com/sotrh/learn-wgpu
// by Benjamin Hansen, licensed under the MIT license
use crate::rect::Rect;
use crate::util::size_of_slice;
use crate::vertex::Vertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

pub struct QuadBufferBuilder {
    vertex_data: Vec<Vertex>,
    index_data: Vec<u32>,
    current_quad: u32,
}

impl QuadBufferBuilder {
    pub fn new() -> Self {
        Self {
            vertex_data: Vec::new(),
            index_data: Vec::new(),
            current_quad: 0,
        }
    }

    pub fn push_rect(self, rect: &Rect) -> Self {
        let coords = rect.top_left_coords;
        self.push_quad(
            coords.x,
            coords.y - rect.height,
            coords.x + rect.width,
            coords.y,
            rect.color,
        )
    }

    pub fn push_quad(
        mut self,
        min_x: f32,
        min_y: f32,
        max_x: f32,
        max_y: f32,
        color: [f32; 3],
    ) -> Self {
        self.vertex_data.extend(&[
            Vertex {
                position: (min_x, min_y).into(),
                color,
            },
            Vertex {
                position: (max_x, min_y).into(),
                color,
            },
            Vertex {
                position: (max_x, max_y).into(),
                color,
            },
            Vertex {
                position: (min_x, max_y).into(),
                color,
            },
        ]);
        self.index_data.extend(&[
            self.current_quad * 4,
            self.current_quad * 4 + 1,
            self.current_quad * 4 + 2,
            self.current_quad * 4,
            self.current_quad * 4 + 2,
            self.current_quad * 4 + 3,
        ]);
        self.current_quad += 1;
        self
    }

    pub fn build(self, device: &wgpu::Device) -> (StagingBuffer, StagingBuffer, u32) {
        (
            StagingBuffer::new(device, &self.vertex_data),
            StagingBuffer::new(device, &self.index_data),
            self.index_data.len() as u32,
        )
    }
}

pub struct StagingBuffer {
    buffer: wgpu::Buffer,
    size: wgpu::BufferAddress,
}

impl StagingBuffer {
    pub fn new<T: bytemuck::Pod + Sized>(device: &wgpu::Device, data: &[T]) -> StagingBuffer {
        StagingBuffer {
            buffer: device.create_buffer_init(&BufferInitDescriptor {
                contents: bytemuck::cast_slice(data),
                usage: wgpu::BufferUsage::COPY_SRC,
                label: Some("Staging Buffer"),
            }),
            size: size_of_slice(data) as wgpu::BufferAddress,
        }
    }

    pub fn copy_to_buffer(&self, encoder: &mut wgpu::CommandEncoder, other: &wgpu::Buffer) {
        encoder.copy_buffer_to_buffer(&self.buffer, 0, other, 0, self.size)
    }
}