---
title: install rabbitmq on ubuntu16.04
date: 2017-12-15
layout: post
mathjax: true
category:
- RabbitMQ
---
## Install Erlang

https://hostpresto.com/community/tutorials/how-to-install-erlang-on-ubuntu-16-04/

````bash
#clean
sudo apt-get update
sudo apt-get -y upgrade
wget https://packages.erlang-solutions.com/erlang-solutions_1.0_all.deb 
sudo dpkg -i erlang-solutions_1.0_all.deb
sudo apt-get update
sudo apt-get install erlang
````

## Install RabbitMQ

To add the Apt repository to your Apt source list directory (/etc/apt/sources.list.d), use:

````bash
echo "deb https://dl.bintray.com/rabbitmq/debian {distribution} main" | sudo tee /etc/apt/sources.list.d/bintray.rabbitmq.list
````

where {distribution} is the name of the Debian or Ubuntu distribution used, e.g. xenial for Ubuntu 16.04, artful for Ubuntu 17.10, or stretch for Debian Stretch.
So, on Ubuntu 16.04 the above command becomes

````
echo "deb https://dl.bintray.com/rabbitmq/debian xenial main" | sudo tee /etc/apt/sources.list.d/bintray.rabbitmq.list
wget -O- https://dl.bintray.com/rabbitmq/Keys/rabbitmq-release-signing-key.asc |
     sudo apt-key add -
sudo apt-get update

sudo apt-get install rabbitmq-server

service rabbitmq-server status
````