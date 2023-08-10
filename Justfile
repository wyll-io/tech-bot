set shell := ["zsh", "-uc"]

base_img_name := "datahearth/tech-bot"
gitea_img_name := "gitea.antoine-langlois.net/" + base_img_name
all_targets := "rust-builder web-builder bot web database"

alias b := build
alias p := push
alias ba := build-all
alias pa := push-all

build TARGET:
  @docker build -t {{base_img_name}}:{{TARGET}} \
    --target {{TARGET}} \
    --cache-to type=inline \
    --cache-from type=registry,ref={{base_img_name}}:{{TARGET}} .
  @docker tag {{base_img_name}}:{{TARGET}} {{gitea_img_name}}:{{TARGET}}

push TARGET: (build TARGET)
  @docker push {{base_img_name}}:{{TARGET}}
  @docker push {{gitea_img_name}}:{{TARGET}}

build-all:
  #!/usr/bin/env zsh
  for target in {{all_targets}}; do
    echo "Building $target";
    docker build -t {{base_img_name}}:$target \
      --target $target \
      --cache-to type=inline \
      --cache-from type=registry,ref={{base_img_name}}:$target .;

    docker tag {{base_img_name}}:$target {{gitea_img_name}}:$target;
  done

push-all: build-all
  #!/usr/bin/env zsh
  for target in {{all_targets}}; do
    echo "Pushing $target";
    docker push {{base_img_name}}:$target;
    docker push {{gitea_img_name}}:$target;
  done
