output "first_ip" {
  value       = vultr_instance.r1net.main_ip
  description = "First IP."
}

output "second_ip" {
  value       = vultr_instance.r2net.main_ip
  description = "Second IP."
}

output "second_ip" {
  value       = vultr_instance.r3net.main_ip
  description = "Third IP."
}
