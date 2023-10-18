package com.plasmoverse.opus;

import java.io.IOException;

public final class OpusDecoder {

    /**
     * Creates a new native opus decoder.
     *
     * @param sampleRate The sample rate of the audio data.
     * @param stereo {@code true} if the audio is in stereo format, {@code false} for mono.
     * @param frameSize Size of the frame.
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native libraries fail to load.
     * @throws OpusException If the opus decoder fail to initialize.
     * @return An instance of the opus decoder.
     */
    public static OpusDecoder create(int sampleRate, boolean stereo, int frameSize) throws IOException, OpusException {
        OpusLibrary.load();

        long pointer = createNative(sampleRate, stereo, frameSize);

        return new OpusDecoder(pointer);
    }

    private static native long createNative(int sampleRate, boolean stereo, int frameSize);


    private final long pointer;

    private OpusDecoder(long pointer) {
        this.pointer = pointer;
    }

    /**
     * Decodes the given encoded audio data into an array of audio samples.
     *
     * @param encoded The encoded audio data to decode.
     * @return An array of audio samples represented as shorts.
     * @throws OpusException If there's an error during the decoding process.
     */
    public short[] decode(byte[] encoded) throws OpusException {
        if (!isOpen()) throw new OpusException("Decoder is closed");

        return decodeNative(encoded);
    }

    /**
     * Resets the opus decoder to its initial state.
     */
    public void reset() {
        if (!isOpen()) return;

        resetNative();
    }

    /**
     * Closes the opus decoder, releasing any allocated resources.
     */
    public void close() {
        if (!isOpen()) return;

        closeNative();
    }

    /**
     * Checks if the opus decoder is currently open and ready for decoding.
     *
     * @return {@code true} if the decoder is open, {@code false} otherwise.
     */
    public boolean isOpen() {
        return pointer > 0;
    }

    private native short[] decodeNative(byte[] encoded);

    private native void resetNative();

    private native void closeNative();
}
