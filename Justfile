set shell := ["zsh", "-uc"]

base_img_name := "datahearth/tech-bot"
gitea_img_name := "gitea.antoine-langlois.net/" + base_img_name
all_targets := "rust-builder web-dependencies bot web database"

alias b := build
alias p := push
alias ba := build-all
alias pa := push-all

gen-graphql:
  @graphql-client introspect-schema http://localhost:8080/graphql > ./bot/graphql/schema.json
  

build TARGET:
  @docker build -t {{base_img_name}}:{{TARGET}}-latest \
    --target {{TARGET}} \
    --cache-to type=inline \
    --cache-from type=registry,ref={{base_img_name}}:{{TARGET}}-latest .
  @docker tag {{base_img_name}}:{{TARGET}}-latest {{gitea_img_name}}:{{TARGET}}-latest

push TARGET: (build TARGET)
  @docker push {{base_img_name}}:{{TARGET}}-latest
  @docker push {{gitea_img_name}}:{{TARGET}}-latest

build-all:
  #!/usr/bin/env zsh
  for target in {{all_targets}}; do
    echo "Building $target";
    docker build -t {{base_img_name}}:$target-latest \
      --target $target \
      --cache-to type=inline \
      --cache-from type=registry,ref={{base_img_name}}:$target-latest .;

    docker tag {{base_img_name}}:$target-latest {{gitea_img_name}}:$target-latest;
  done

push-all: build-all
  #!/usr/bin/env zsh
  for target in {{all_targets}}; do
    echo "Pushing $target";
    docker push {{base_img_name}}:$target-latest;
    docker push {{gitea_img_name}}:$target-latest;
  done
