plugins {
    java
}

repositories {
    mavenCentral()
}

dependencies {
    testCompileOnly(libs.junit.api)
    testAnnotationProcessor(libs.junit.api)
    testRuntimeOnly(libs.junit.engine)
}

tasks {
    test {
        useJUnitPlatform()
    }
}
