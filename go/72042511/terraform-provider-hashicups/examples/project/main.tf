terraform {
  required_providers {
    hashicups = {
      version = "0.2.0"
      source = "hashicorp.com/edu/hashicups"
    }
  }
}

data "hashicups_projects" "all" {
}

# Returns projects
output "projects" {
  value = data.hashicups_projects.all
}

output "project" {
  value = {
    members = data.hashicups_projects.all.members
    owners = data.hashicups_projects.all.owners
  }
}
