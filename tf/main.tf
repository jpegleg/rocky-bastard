resource "vultr_instance" "r1net" {
    plan = "vc2-2c-2gb"
    region = "atl"
    os_id = 448
}

resource "vultr_instance" "r2net" {
    plan = "vc2-2c-2gb"
    region = "atl"
    os_id = 448
}

resource "vultr_instance" "r3net" {
    plan = "vc2-2c-2gb"
    region = "atl"
    os_id = 448
}
