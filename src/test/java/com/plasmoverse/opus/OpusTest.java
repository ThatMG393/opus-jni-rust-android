package com.plasmoverse.opus;

import org.junit.jupiter.api.Test;

public final class OpusTest {

    @Test
    public void decode() throws Exception {
        /**
         * Encoding
         */
        short[] rawSamples = new short[960];

        // Creates a new encoder in mono with 1024 mtu size and application mode VOIP
        OpusEncoder encoder = OpusEncoder.create(48_000, false, 960, OpusMode.VOIP);

        // Sets encoder bitrate to 50k
        encoder.setBitrate(50_000);

        // Encodes the raw audio samples
        byte[] encoded = encoder.encode(rawSamples);

        // Resets the encoder state
        encoder.reset();

        // Closes the encoder, releasing allocated resources
        encoder.close();

        /**
         * Decoding
         */

        // Creates a new decoder in mono with 960 buffer size
        // buffer_size should be frame_size * channels
        OpusDecoder decoder = OpusDecoder.create(48_000, false, 960);

        // Decodes the encoded audio data into an audio samples
        short[] decoded = decoder.decode(encoded);

        // Resets the decoder state
        decoder.reset();

        // Closes the decoder, releasing allocated resources
        decoder.close();
    }
}
