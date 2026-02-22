# Migrations à faire

## Gradle IntelliJ Plugin 1.x → 2.x

**Priorité**: Moyenne
**Complexité**: Élevée (breaking changes)

### Changements requis

Le plugin `org.jetbrains.intellij` v1.x a été remplacé par `org.jetbrains.intellij.platform` v2.x.

#### `build.gradle.kts` — différences principales

```kotlin
// AVANT (1.x)
plugins {
    id("org.jetbrains.intellij") version "1.17.4"
}
intellij {
    version.set("2024.1.7")
    type.set("IC")
}

// APRÈS (2.x)
plugins {
    id("org.jetbrains.intellij.platform") version "2.x.x"
}
intellijPlatform {
    intellijIdeaCommunity("2024.1.7")
}
dependencies {
    intellijPlatform {
        instrumentationTools()
    }
}
```

#### Kotlin 1.9.25 → 2.1.x

Mettre à jour en même temps que la migration IntelliJ plugin :
```kotlin
id("org.jetbrains.kotlin.jvm") version "2.1.10"
```

### Références

- [Migration guide officielle](https://plugins.jetbrains.com/docs/intellij/tools-intellij-platform-gradle-plugin-migration.html)
- [Changelog plugin 2.x](https://github.com/JetBrains/intellij-platform-gradle-plugin/blob/main/CHANGELOG.md)
