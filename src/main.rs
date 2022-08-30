#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rand::Rng;

fn main() {

    #[get("/calculateDisselUsageForDistance/<distance>/<_year_of_production>/<fuel_usage_per100_km>")]
    fn calc_fuel_usage(distance: u64, _year_of_production: u64, fuel_usage_per100_km: u64) -> String {
        let fuel_usage = fuel_usage_per100_km * distance / 100;
        format!{"FuelUsage is {}", fuel_usage}
    }

    type F = fraction::Fraction;

    #[get("/probabilityOfUnitInjectorFail/<_vin>")]
    fn calc_fail_prob(_vin: String) -> String {
        let random_number = rand::thread_rng().gen_range(0..101);
        let fail_probability = F::from(random_number) / F::from(100);
        format!{"There is {:.2} fail probability", fail_probability}
    }

    rocket::ignite()
        .mount(
            "/",
            routes![calc_fuel_usage, calc_fail_prob],
        )
        .launch();
}