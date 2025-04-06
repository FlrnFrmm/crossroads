// A module for Crossroads functions

package main

import (
	"context"
	"dagger/crossroads/internal/dagger"
	"fmt"
	"strings"
	"time"
)

type Crossroads struct{}

// Get the next version to build
func (m *Crossroads) Version(ctx context.Context,
	// +ignore=[".cicd","dagger.json",".env","/target","justfile","Cargo.lock"]
	// +defaultPath="/"
	directory *dagger.Directory,
) (string, error) {

	return dag.Container().
		From("rust:latest").
		WithDirectory("/crossroads", directory).
		WithWorkdir("/crossroads").
		WithExec([]string{"cargo", "install", "cocogitto"}).
		WithExec([]string{"sh", "-c", "cog bump --dry-run --auto --skip-ci --skip-untracked"}).
		Stdout(ctx)
}

// Build the current version
func (m *Crossroads) Build(ctx context.Context,
	// +default="linux/amd64"
	platform string,
	// +defaultPath="/"
	directory *dagger.Directory) *dagger.Directory {
	return dag.Container(dagger.ContainerOpts{Platform: dagger.Platform(platform)}).
		From("alpine:latest").
		WithDirectory("/crossroads", directory).
		WithWorkdir("/crossroads").
		WithExec([]string{"apk", "add", "cargo"}).
		WithExec([]string{"sh", "-c", "cargo", "build", "--release"}).
		WithExec([]string{"sh", "-c", "cargo", "doc"}).
		Directory("./target/release")
}

// Containerize the build
func (m *Crossroads) Containerize(ctx context.Context,
	// +default="linux/amd64"
	platform string,
	// +ignore=[".cicd",".git", "**/.gitignore","dagger.json",".env","/target","justfile","Cargo.lock"]
	// +defaultPath="/"
	directory *dagger.Directory) *dagger.Container {

	version, error := m.Version(ctx, directory)

	if error != nil {

	}

	build := m.Build(ctx, platform, directory)

	// Add bump logic

	return dag.Container(dagger.ContainerOpts{Platform: dagger.Platform(platform)}).
		From("alpine:latest").
		WithDirectory("/build", build).
		WithWorkdir("/build").
		WithLabel("org.opencontainers.image.authors", "Crossroads Contributors").
		WithLabel("org.opencontainers.image.vendor", "Crossroads Developer Team").
		WithLabel("org.opencontainers.image.source", "https://github.com/FlrnFrmm/crossroads").
		WithLabel("org.opencontainers.image.documentation", "https://docs.rs/crossroads/latest/crossroads").
		WithLabel("org.opencontainers.image.created", fmt.Sprintf("%v", time.Now().Format(time.RFC1123))).
		WithLabel("org.opencontainers.image.version", strings.Trim(fmt.Sprintf("v%v", version), "\n"))
}

// Publish a Tag based with default latest or the version,
func (m *Crossroads) PublishTag(ctx context.Context,
	// +default=false
	dev bool,
	// +default="ghcr.io"
	registry string,
	application string,
	token *dagger.Secret,
	// +ignore=[".cicd","dagger.json",".env","/target","justfile","Cargo.lock"]
	// +defaultPath="/"
	directory *dagger.Directory) (string, error) {

	ver := "dev"

	if !dev {
		version, err := m.Version(ctx, directory)
		ver = version
		if err != nil {
			return "", err
		}
	}

	if ver != "latest" && ver != "dev" {
		if !strings.Contains(ver, "v") {
			ver = strings.Trim(fmt.Sprintf("v%v", ver), "\n")
		}
	}

	return dag.Container().
		From("alpine/git:latest").
		WithDirectory("/crossroads", directory).
		WithWorkdir("/crossroads").
		WithExec([]string{"sh", "-c", fmt.Sprintf("git tag %v", ver)}).
		WithExec([]string{"sh", "-c", fmt.Sprintf("git push origin tag %v", ver)}).
		Stdout(ctx)
}

func (m *Crossroads) Publish(ctx context.Context,
	// +default="app"
	application string,
	// +default="https://ghcr.io/FlrnFrmm/crossroads"
	url string,
	// +default="linux/amd64"
	platform string,
	// +default="ghcr.io"
	registry string,
	// +ignore=[".cicd","dagger.json",".env","/target","justfile","Cargo.lock"]
	// +defaultPath="/"
	directory *dagger.Directory,
	secret *dagger.Secret,
) (string, error) {

	version, error := m.Version(ctx, directory)

	if error != nil {
		return "", error
	}

	address := fmt.Sprintf("%v:v%v", url, strings.Trim(version, "\n"))

	return m.Containerize(ctx, platform, directory).
		WithRegistryAuth(registry, application, secret).
		Publish(ctx, address, dagger.ContainerPublishOpts{})
}

// Publish to ghcr.io
func (m *Crossroads) PublishAll(ctx context.Context,
	// +default="app"
	application string,
	// +default="https://ghcr.io/FlrnFrmm/crossroads"
	url string,
	// +default="ghcr.io"
	registry string,
	// +ignore=[".cicd","dagger.json",".env","/target","justfile","Cargo.lock"]
	// +defaultPath="/"
	directory *dagger.Directory,
	secret *dagger.Secret,
) (string, error) {

	platforms := []string{"linux/amd64", "linux/arm64"}
	platformVariants := make([]*dagger.Container, 0, len(platforms))

	for _, platform := range platforms {

		ctr := m.Containerize(ctx, platform, directory)
		platformVariants = append(platformVariants, ctr)

	}

	version, error := m.Version(ctx, directory)
	if error != nil {
		return "", error
	}

	address := fmt.Sprintf("%v:v%v", url, strings.Trim(version, "\n"))

	return dag.Container().
		WithRegistryAuth(address, application, secret).
		Publish(ctx, address, dagger.ContainerPublishOpts{
			PlatformVariants: platformVariants,
		})
}
