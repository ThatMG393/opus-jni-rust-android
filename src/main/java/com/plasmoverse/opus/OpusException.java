package com.plasmoverse.opus;

/**
 * Exception indicates issues related to opus.
 */
public class OpusException extends Exception {

    public OpusException() {
        super();
    }

    public OpusException(String message) {
        super(message);
    }

    public OpusException(String message, Throwable cause) {
        super(message, cause);
    }

    public OpusException(Throwable cause) {
        super(cause);
    }
}
