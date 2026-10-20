terraform {
  required_providers {
    aws = {
      source = "hashicorp/aws"
      version = "~> 6.69.1" 
    }
  }
}

provider "aws" {
  region = "us-east-1"
}

data "aws_ami" "ubuntu" {
  most_recent = true 
  owners = ["099720109477"] # Canonical

  filter {
    name = "name"
    values = ["ubuntu/images/hvm-ssd-gp3/ubuntu-noble-24.04-arm64-server-*"]
  }
}
 
resource "aws_instance" "example" {
  ami = data.aws_ami.ubuntu.id
  instance_type = "t4g.nano"
}

