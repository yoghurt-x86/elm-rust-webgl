#!/bin/bash

session="rust"

tmux new-session -d -s $session

tmux rename-window -t $session:1 'shells'
tmux split-window -v -t $session:1
tmux split-window -v -t $session:1

tmux send-keys -t $session:1.0 'nix-shell' Enter
tmux send-keys -t $session:1.1 'nix-shell' Enter
tmux send-keys -t $session:1.2 'nix-shell' Enter

#tmux new-window -t $session:2 -n 'vim'
#tmux send-keys -t $session:2 'nix-shell' Enter

sleep 3

#tmux send-keys -t $session:2 'nvim' Enter

tmux send-keys -t $session:1.0 'python -m http.server' Enter
tmux send-keys -t $session:1.0 './build.sh' Enter

tmux send-keys -t $session:1.1 './listen.sh' Enter

tmux send-keys -t $session:1.2 'cd elm' Enter
tmux send-keys -t $session:1.2 './listen.sh' Enter

