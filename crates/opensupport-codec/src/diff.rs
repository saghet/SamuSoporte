use crate::error::CodecError;
use lz4_flex::{compress_prepend_size, decompress_size_prepended};

#[derive(Debug, Clone)]
pub struct DiffFrame {
    pub is_keyframe: bool,
    pub width: u32,
    pub height: u32,
    pub compressed_data: Vec<u8>,
}

pub struct FrameCompressor {
    prev_frame: Vec<u8>,
    width: u32,
    height: u32,
    diff_buffer: Vec<u8>,
}

impl FrameCompressor {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize; // RGBA 4 bytes por píxel
        Self {
            prev_frame: vec![0u8; size],
            width,
            height,
            diff_buffer: vec![0u8; size],
        }
    }

    /// Comprime un frame RGBA completo o genera un diferencial contra el anterior
    pub fn compress_frame(&mut self, current_frame: &[u8], force_keyframe: bool) -> Result<DiffFrame, CodecError> {
        let expected_size = (self.width * self.height * 4) as usize;
        if current_frame.len() != expected_size {
            return Err(CodecError::BufferTooSmall {
                expected: expected_size,
                provided: current_frame.len(),
            });
        }

        if force_keyframe || self.prev_frame.is_empty() {
            // Guardar frame completo como base y comprimir
            self.prev_frame.copy_from_slice(current_frame);
            let compressed = compress_prepend_size(current_frame);

            Ok(DiffFrame {
                is_keyframe: true,
                width: self.width,
                height: self.height,
                compressed_data: compressed,
            })
        } else {
            // Generar XOR diferencial
            let mut diff_count = 0;
            for i in 0..expected_size {
                let diff = current_frame[i] ^ self.prev_frame[i];
                self.diff_buffer[i] = diff;
                if diff != 0 {
                    diff_count += 1;
                }
            }

            // Si más del 70% de la imagen cambió, emitir KeyFrame para eficiencia
            if diff_count > (expected_size * 7 / 10) {
                self.prev_frame.copy_from_slice(current_frame);
                let compressed = compress_prepend_size(current_frame);

                Ok(DiffFrame {
                    is_keyframe: true,
                    width: self.width,
                    height: self.height,
                    compressed_data: compressed,
                })
            } else {
                // Actualizar buffer de referencia con el nuevo frame
                self.prev_frame.copy_from_slice(current_frame);
                let compressed = compress_prepend_size(&self.diff_buffer);

                Ok(DiffFrame {
                    is_keyframe: false,
                    width: self.width,
                    height: self.height,
                    compressed_data: compressed,
                })
            }
        }
    }
}

pub struct FrameDecompressor {
    canvas: Vec<u8>,
    width: u32,
    height: u32,
}

impl FrameDecompressor {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height * 4) as usize;
        Self {
            canvas: vec![0u8; size],
            width,
            height,
        }
    }

    pub fn decompress_frame(&mut self, diff: &DiffFrame) -> Result<&[u8], CodecError> {
        let expected_size = (self.width * self.height * 4) as usize;
        let decompressed = decompress_size_prepended(&diff.compressed_data)
            .map_err(|e| CodecError::Decompression(e.to_string()))?;

        if decompressed.len() != expected_size {
            return Err(CodecError::BufferTooSmall {
                expected: expected_size,
                provided: decompressed.len(),
            });
        }

        if diff.is_keyframe {
            self.canvas.copy_from_slice(&decompressed);
        } else {
            // Aplicar XOR diferencial para reconstruir el frame completo
            for i in 0..expected_size {
                self.canvas[i] ^= decompressed[i];
            }
        }

        Ok(&self.canvas)
    }
}
