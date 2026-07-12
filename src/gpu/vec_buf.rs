use wgpu::Buffer;

pub struct VecBuf<T>
where
    T: encase::ShaderType,
{
    buffer: Buffer,
    capacity: u64, // number of T elements the buffer can currently hold
    scratch: encase::StorageBuffer<Vec<u8>>,
    phantom: std::marker::PhantomData<T>,
}

impl<T: encase::ShaderType> VecBuf<T> {
    pub fn with_capacity(device: &wgpu::Device, capacity: u64) -> Self {
        let buffer = Self::alloc(device, capacity);
        Self {
            buffer,
            capacity,
            scratch: encase::StorageBuffer::<Vec<u8>>::new(Vec::new()),
            phantom: std::marker::PhantomData,
        }
    }

    fn alloc(device: &wgpu::Device, capacity: u64) -> Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: <T as encase::ShaderType>::min_size().get() * capacity.max(1),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn buf(&self) -> &Buffer {
        &self.buffer
    }
    #[allow(unused)]
    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    /// Grows the buffer if `needed` exceeds current capacity.
    /// Returns `true` if a new buffer was allocated — callers MUST
    /// rebuild any bind group referencing `buf()` when this is true.
    #[must_use]
    pub fn ensure_capacity(&mut self, device: &wgpu::Device, needed: u64) -> bool {
        if needed <= self.capacity {
            return false;
        }
        let new_capacity = needed.max(self.capacity * 2).max(1);
        self.buffer = Self::alloc(device, new_capacity);
        self.capacity = new_capacity;
        true
    }

    /// Writes `data` (anything encase can serialize as this buffer's
    /// element type, e.g. `&Vec<&T>`) into the buffer, growing first
    /// if needed. `len` is the element count of `data`.
    /// Returns `true` if the buffer was reallocated.
    #[must_use]
    pub fn write<D>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: &D,
        len: u64,
    ) -> bool
    where
        D: encase::internal::WriteInto + ?Sized + encase::ShaderType,
    {
        let grew = self.ensure_capacity(device, len);
        self.scratch.write(data).unwrap();
        let needed_bytes = (<T as encase::ShaderType>::min_size().get() * len) as usize;
        queue.write_buffer(&self.buffer, 0, &self.scratch.as_ref()[..needed_bytes]);
        grew
    }
}
