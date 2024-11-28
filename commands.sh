#!/usr/bin/env sh

ty=$1
flag=$2

   if [ "$ty" = "client" ]; then
     echo "Run $ty $flag command"
             cd client_ts

           yarn run $flag


elif [ "$ty" = "rust" ]
then
  echo "Run $ty $flag command"
build()
{
  cargo build-bpf
  }
fmt()
{

cargo fmt --all
}
lint()
{
  cargo clippy --all && cargo fix --tests --all-features --allow-dirty
  }
deploy()
{
  cargo build-bpf --manifest-path=./program/Cargo.toml --bpf-out-dir=./dist/program
  solana airdrop 1
  solana program deploy dist/program/example_program.so
}
  case $flag in
           "build")
             cd program
             build
             ;;
         "fmt")
         cd program
         fmt
         ;;
         "lint")
         cd program;
         lint
         ;;
       "pre_commit")
                build
                fmt
                lint
                ;;
         "deploy")
      deploy
         ;;
         "redeploy")
       echo "" | solana-keygen new --outfile dist/program/example_program-keypair.json --force
       deploy
         ;;
   esac

else echo "client commands:
\n client initialize (initialize campaign )
\n client create (create campaign )
\n client buy_ticket
\n client draw (get winner)
\nclient claim  (claim reward)
_____________________
rust commands:
\n rust built
\n rust fmt
\n rust lint
\n rust pre_commit
\n rust deploy
\n rust redeploy"
  fi