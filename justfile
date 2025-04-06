set dotenv-load
set dotenv-required

debug:
    dagger core

version:
    dagger call version

build:
    dagger call build

containerize:
    dagger call containerize

tag:
    dagger call publish-tag \
        --application=$APPLICATION \
        --token=env:TOKEN

publish:
    dagger call publish \
        --application=$APPLICATION \
        --token=env:TOKEN

publish-all:
    dagger call publish-all \
        --application=$APPLICATION \
        --token=env:TOKEN

release: publish-all tag
