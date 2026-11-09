#[cfg(test)]
mod tests {
    use opensupport_codec::{FrameCompressor, FrameDecompressor};

    #[test]
    fn test_codec_keyframe_and_diff_reconstruction() {
        let width = 64;
        let height = 64;
        let size = (width * height * 4) as usize;

        let mut compressor = FrameCompressor::new(width, height);
        let mut decompressor = FrameDecompressor::new(width, height);

        // Frame 1: Fondo azul (RGBA: 0, 0, 255, 255)
        let mut frame1 = vec![0u8; size];
        for chunk in frame1.chunks_exact_mut(4) {
            chunk[0] = 0;
            chunk[1] = 0;
            chunk[2] = 255;
            chunk[3] = 255;
        }

        let keyframe = compressor.compress_frame(&frame1, true).unwrap();
        assert!(keyframe.is_keyframe);

        let reconstructed1 = decompressor.decompress_frame(&keyframe).unwrap();
        assert_eq!(reconstructed1, &frame1[..], "Frame 1 reconstruido no coincide");

        // Frame 2: Cambio menor (un cuadro rojo en la esquina superior)
        let mut frame2 = frame1.clone();
        for i in 0..16 {
            let offset = i * 4;
            frame2[offset] = 255;
            frame2[offset + 1] = 0;
            frame2[offset + 2] = 0;
            frame2[offset + 3] = 255;
        }

        let diff_frame = compressor.compress_frame(&frame2, false).unwrap();
        assert!(!diff_frame.is_keyframe, "El cambio leve debió generar un diferencial");

        let reconstructed2 = decompressor.decompress_frame(&diff_frame).unwrap();
        assert_eq!(reconstructed2, &frame2[..], "Frame 2 diferencial reconstruido no coincide");
    }
}
