plugins {
    id("java")
    id("org.jetbrains.kotlin.jvm") version "1.9.25"
    id("org.jetbrains.intellij") version "1.17.4"
    id("org.jetbrains.changelog") version "2.2.1"
}

group = "com.sybernatus"
version = "0.13.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("com.fasterxml.jackson.core:jackson-databind:2.18.+")
    implementation("com.fasterxml.jackson.dataformat:jackson-dataformat-yaml:2.18.+")
    implementation("net.coobird:thumbnailator:0.4.20")
}

// Configure Gradle IntelliJ Plugin
// Read more: https://plugins.jetbrains.com/docs/intellij/tools-gradle-intellij-plugin.html
intellij {
    version.set("2024.1.7")
    type.set("IC") // Target IDE Platform

    plugins.set(listOf(/* Plugin Dependencies */))
}

tasks {
    // Set the JVM compatibility versions
    withType<JavaCompile> {
        sourceCompatibility = "17"
        targetCompatibility = "17"
    }
    withType<org.jetbrains.kotlin.gradle.tasks.KotlinCompile> {
        kotlinOptions.jvmTarget = "17"
    }

    patchPluginXml {
        sinceBuild.set("241")
        untilBuild.set("")
        changeNotes.set(provider {
            changelog.renderItem(
                changelog.get(project.version.toString()),
                org.jetbrains.changelog.Changelog.OutputType.HTML
            )
        })
        pluginDescription.set(
            provider {
                val readme = file("build/README.html").readText()
                readme
            }
        )
    }

    signPlugin {
        certificateChain.set(System.getenv("CERTIFICATE_CHAIN"))
        privateKey.set(System.getenv("PRIVATE_KEY"))
        password.set(System.getenv("PRIVATE_KEY_PASSWORD"))
    }

    publishPlugin {
        token.set(System.getenv("IDEA_PUBLISH_TOKEN"))
    }
}

changelog {
    version.set(project.version.toString())
    path.set("${project.projectDir}/CHANGELOG.md")
}