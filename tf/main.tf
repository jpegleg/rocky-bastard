resource "vultr_instance" "r1net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
}

resource "vultr_instance" "r2net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
}

resource "vultr_instance" "r3net" {
    plan = "vc2-1c-1gb"
    region = "atl"
    os_id = 448
}
