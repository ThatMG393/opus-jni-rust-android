# opus-jni-rust
Simple JNI wrapper for the [Opus](https://opus-codec.org/) using [Rust](https://www.rust-lang.org/) 🚀

### Adding dependency to the project
<img alt="version" src="https://img.shields.io/badge/dynamic/xml?label=%20&query=/metadata/versioning/versions/version[not(contains(text(),'%2B'))][last()]&url=https://repo.plasmoverse.com/releases/com/plasmoverse/opus-jni-rust/maven-metadata.xml">

```kotlin
repositories {
    maven("https://repo.plasmoverse.com/releases")
}

dependencies {
    implementation("com.plasmoverse:opus-jni-rust:$version")
}
```

### Simple usage
Sample code from [OpusTest.java](https://github.com/plasmoapp/opus-jni-rust/blob/main/src/test/java/com/plasmoverse/opus/OpusTest.java)
```java
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
OpusDecoder decoder = OpusDecoder.create(48_000, false, 960);

// Decodes the encoded audio data into an audio samples
short[] decoded = decoder.decode(encoded);

// Resets the decoder state
decoder.reset();

// Closes the decoder, releasing allocated resources
decoder.close();
```
