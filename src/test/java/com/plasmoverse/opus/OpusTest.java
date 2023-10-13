package com.plasmoverse.opus;

import org.junit.jupiter.api.Test;

public final class OpusTest {

    @Test
    public void decode() throws Exception {
        OpusEncoder encoder = OpusEncoder.create(48_000, false, 960, OpusMode.VOIP);

        byte[] encoded = encoder.encode(new short[960]);

        encoder.setBitrate(50_000);
        encoder.reset();
        encoder.close();

        OpusDecoder decoder = OpusDecoder.create(48_000, false, 1200);

        short[] decoded = decoder.decode(encoded);

        decoder.reset();
        decoder.close();
    }
}
