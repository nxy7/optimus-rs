![Optimus Prime](https://www.dexerto.com/cdn-cgi/image/width=3840,quality=75,format=auto/https://editors.dexerto.com/wp-content/uploads/2023/04/21/Transformers-1.jpg)

# This is forked repo of optimus
I was trying to create optimus using golang, but ended up unhappy with how it turns out. This repository is made as experiment to get a feel for the same project structure built with Rust. 

--------------------------------

Monorepo management tool that's extensible and will fit any workflow.

# Key Features:

- Simplified monorepo management. ☑
- Doesn't use obscure languages for configuration. ☑
- Extensible using any shell that's available on the system. ☑
- Discoverable: allows newcomers to easily find out all parts of the system they can interact with. ☑
- Smart testing makes CI pipelines faster. ☑
- Post command hooks allow you to run script after specific command. ☒
- Integrates with other tools like telepresence. ☑
- Works with Docker Compose, Kubernetes and standalone apps. ☑
- Composable: configuration can be split into many files. ☑
- Commands work from any directory within repository. ☑

# Why use shell to extend Optimus capabilities

I'm not huge fan of shell scripting, but when it comes to managing whole project it just makes sense. Shell scripting languages treat external executables as first class objects, which makes it easy to quickly adapt Optimus to any workflow. Most developers are familliar with at least one shell scripting language (even if only to launch their apps), so allowing them to reuse existing knowledge is much better than forcing them into using specific tech (many simmilar tools use python/starlark/lua). If you want to use sane scripting language while configuring Optimus I recommend `nushell`.

# Optimus works well together with other tools

If you really think about it, Optimus is mostly made to organize scripts that would otherwise be loosely coupled and sitting in random places of your project. If you think about it as script runner with some extra perks it makes sense that you can extend it using any other available tools. Use any container builder or testing framework you want, in the end Optimus can use anything that's available on your system. This approach works great with *nix* projects, where you can make sure that all tools used with Optimus are pinned at just the right versions.

# Optimus configuration

Optimus configuration is very simple to understand really, it only has 2 concepts right now - services and commands.

## Command

Command accepts 2 notations, short and long one.
Short:
```yaml
someCommand: |
  echo "it's regular bash command here"
```
Long one (with all possible fields):
```yaml
someCommand: 
  description: |
    Your command description that shows up when you run 'optimus'
  shell: nu -c
  run:
    print "now it's nushell command and you can use"; sleep 1sec; print "nushell syntax instead";
anotherCommand:
  description: |
    This command run's bash script from file, not as -c
  file: ./path_to_script.sh
```

Short command is the same as having command with no description and `bash` as shell. One note though, you cannot have both `file` and `run` fields in the same command. Any config entry besides specified keywords such as `services`, `global` or `test` will be treated as command. This allows you to group all mundane tasks you're usually doing inside your repository and run them with `optimus someCommand`


## Services

Services are meant as a way to gather microservice specific commands. All commands from services have the same *syntax* as commands, but some of them are treated specially. For example running `optimus test` will run "test" command of all services, cache results and if the content of your services doesn't change it will not rerun those tests on subsequent runs of `optimus test`. 

Example service:
```yaml
services:
  frontendService:
    # if you want to name your service differently than the folder containing it, use 'root' propety so commands work fine
    root: ./frontend
    dev: echo "it's just 'command' so you can use long or short syntax here"
    test: echo "same as above"
  backend:
    dev: |
      echo "Commands are ran off their microservices directories"
      ls
      echo "ls above will output dir of $PROJECT_ROOT/backend folder"
    build: |
      echo "you are responsible for running docker image build commands"
      echo "if you run 'optimus build' optimus will run build commands of all services cocurrently"
```

# Example configurations

## Standalone App

```yaml
# invoked via 'optimus frontend dev'
services:
  frontend:
    dev: |
      pnpm i && pnpm dev

  backend:
    dev: |
      cargo run

# invoked via 'optimus utilityFunction'
utilityFunction:
  description: |
    Function that does some repository related work
  run: |
    echo "Working..."
```

## Docker Compose

```yaml
# invoked via 'optimus start'
start: |
  docker compose -f compose.yml -f compose.dev.yml up -d

# invoked via 'optimus stop'
stop: |
  docker compose -f compose.yml -f compose.dev.yml down

# if any service contains 'build' you can use 'optimus build' to run builds in all services cocurrently
# it works the same for 'test' command
services:
  frontend:
    dev: |
      pnpm i && pnpm dev
    build: |
      docker build .

  backend:
    dev: |
      cargo run
    build: |
      docker build .
```

## Kubernetes

Here's part of configuration that I'm using for the service I'm developing that runs on Kubernetes
```yaml
# 'optimus start' invokes external script, which is useful if you have more logic to some step
start:
  description: |
    Start streampai application
  run: |
    nu ./scripts/kubernetes.nu init

clean: 
  description: |
    Delete all project resources
  run: |
    nu ./scripts/kubernetes.nu purge

telepresence-reset: 
  description: |
    Telepresence sometimes hangs and needs to be reset using this command
  run: |
    nu ./scripts/kubernetes.nu reset-telepresence     

services:
  frontend:
    dev: |
      telepresence intercept frontend --port 3000:http --mechanism tcp --namespace streampai
      pnpm i && pnpm dev
    build: |
      docker run .
  backend/main:
    root: ./backend
    dev: |
      telepresence intercept backend --port 7000:http --mechanism tcp --namespace streampai
      cargo run --bin main
    build: |
      docker run . -f Dockerfile.main
  backend/streamchat:
    root: ./backend
    dev: |
       cargo run --bin streamchat
```

# Caching

Optimus can cache test commands for you, which shortens feedback loop and makes CI significantly faster. Cache is saved in `optimus.cache` file in the root of your project, so if you don't add this file to `.gitignore` it'll be shared with your CI and other coworkers. 

# How to install

The easiest way to install this software would be using output of the flake from this repo and that's primarily how I'm using it in my other projects.

# Why 'Optimus'

For some reason I thought that if You compared microservices to transformers, then tool managing them should be called by the name of transformers leader. The name sounds familiar, is easy to remember and easy to alias ('op'). 

# TODO
- clean up codebase
- make init command that would create optimus files based on project structure (detect package.json, cargo.toml etc.)
- allow using env variables or .env file
