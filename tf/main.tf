resource "vultr_ssh_key" "initkey" {
  name = "initkey"
  ssh_key = "public key goes here"
}

resource "vultr_instance" "r1net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
    ssh_key_ids = ["${vultr_ssh_key.initkey.id}"]
}

resource "vultr_instance" "r2net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
    ssh_key_ids = ["${vultr_ssh_key.initkey.id}"]
}

resource "vultr_instance" "r3net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
    ssh_key_ids = ["${vultr_ssh_key.initkey.id}"]
}
