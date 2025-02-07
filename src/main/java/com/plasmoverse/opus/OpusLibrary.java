package com.plasmoverse.opus;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.nio.file.AccessDeniedException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.StandardCopyOption;

public final class OpusLibrary {

    private static boolean LOADED;

    /**
     * Extracts the native library to the temp dir and loads it.
     *
     * @throws IOException If an error occurs while extracting the native library.
     * @throws UnsatisfiedLinkError If the native library fail to load.
     */
    public static void load() throws IOException {
        if (LOADED) return;

        File temporaryDir = Files.createTempDirectory("opus-jni-rust").toFile();
        temporaryDir.deleteOnExit();

        String libraryName = getPlatformLibraryFileName("opus_jni_rust");
        String platformFolder = getPlatformFolderName();
        String nativeLibraryPath = String.format("/natives/%s/%s", platformFolder, libraryName);

        InputStream source = OpusLibrary.class.getResourceAsStream(nativeLibraryPath);
        if (source == null) {
            throw new IOException("Couldn't find the native library for " + platformFolder + ": " + nativeLibraryPath);
        }

        Path destination = temporaryDir.toPath().resolve(libraryName);
        try {
            Files.copy(source, destination, StandardCopyOption.REPLACE_EXISTING);
        } catch (AccessDeniedException ignored) {
        }
        System.load(destination.toFile().getAbsolutePath());

        LOADED = true;
    }

    private static String getPlatformFolderName() {
        return String.format(
                "%s-%s",
                getPlatformName(),
                getPlatformArch()
        );
    }

    private static String getPlatformName() {
        String systemName = System.getProperty("os.name").toLowerCase();

        if (systemName.contains("nux") || systemName.contains("nix")) {
            return "linux";
        } else if (systemName.contains("mac")) {
            return "mac";
        } else if (systemName.contains("windows")) {
            return "win";
        } else {
            throw new IllegalStateException("System is not supported: " + systemName);
        }
    }

    private static String getPlatformArch() {
        String systemArch = System.getProperty("os.arch").toLowerCase();

        switch (systemArch) {
            case "i386":
            case "i486":
            case "i586":
            case "i686":
            case "x86":
            case "x86_32":
                return "x86";
            case "amd64":
            case "x86_64":
            case "x86-64":
                return "x64";
            case "aarch64":
                return "aarch64";
            default:
                return OS_ARCH;
        }
    }

    private static String getPlatformLibraryFileName(String library) {
        switch (getPlatformName()) {
            case "linux":
                return "lib" + library + ".so";
            case "mac":
                return "lib" + library + ".dylib";
            case "windows":
                return library + ".dll";
            default:
                throw new IllegalStateException("System is not supported: " + systemName);
        }
    }

    private OpusLibrary() {
    }
}
