package com.plasmoverse.opus;

import java.io.IOException;

public final class OpusEncoder {

    /**
     * Creates a new native opus encoder.
     *
     * @param sampleRate The sample rate of the audio data.
     * @param stereo {@code true} if the audio is in stereo format, {@code false} for mono.
     * @param mtuSize A max size of the output array.
     * @param mode The opus application mode.
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native libraries fail to load.
     * @throws OpusException If the opus decoder fail to initialize.
     * @return An instance of the opus decoder.
     */
    public static OpusEncoder create(int sampleRate, boolean stereo, int mtuSize, OpusMode mode) throws IOException, OpusException {
        OpusLibrary.load();

        long pointer = createNative(sampleRate, stereo, mtuSize, mode.getApplication());

        return new OpusEncoder(pointer);
    }

    private static native long createNative(int sampleRate, boolean stereo, int mtuSize, int mode);


    private final long pointer;

    private OpusEncoder(long pointer) {
        this.pointer = pointer;
    }

    /**
     * Encodes the given audio samples into an opus format represented as an array of bytes.
     *
     * @param samples The audio samples to encode.
     * @return An array of bytes containing the encoded audio data.
     * @throws OpusException If there's an error during the decoding process.
     */
    public byte[] encode(short[] samples) throws OpusException {
        if (!isOpen()) throw new OpusException("Encoder is closed");

        return encodeNative(samples);
    }

    /**
     * Resets the audio decoder to its initial state.
     */
    public void reset() {
        if (!isOpen()) return;

        resetNative();
    }

    /**
     * Closes the audio decoder, releasing any allocated resources.
     */
    public void close() {
        if (!isOpen()) return;

        closeNative();
    }

    /**
     * Sets the bitrate for opus encoder.
     * <br/>
     * Supported values:
     * <ul>
     *     <li>-1000 for auto</li>
     *     <li>-1 for max</li>
     *     <li>[500-512_000]</li>
     * </ul>
     * @param bitrate The bitrate to set.
     */
    public void setBitrate(int bitrate) {
        if (!isOpen()) return;

        setBitrateNative(bitrate);
    }

    /**
     * Gets the opus encoder bitrate.
     * @return The bitrate.
     */
    public int getBitrate() throws OpusException {
        if (!isOpen()) throw new OpusException("Encoder is closed");
        return getBitrateNative();
    }

    /**
     * Checks if the audio decoder is currently open and ready for decoding.
     *
     * @return {@code true} if the decoder is open, {@code false} otherwise.
     */
    public boolean isOpen() {
        return pointer > 0;
    }

    private native byte[] encodeNative(short[] samples);

    private native void resetNative();

    private native void closeNative();

    private native void setBitrateNative(int bitrate);

    private native int getBitrateNative();
}
