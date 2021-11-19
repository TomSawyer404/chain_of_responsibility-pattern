#[derive(Debug)]
struct Patient {
    name: String,
    registeration_done: bool,
    doctor_checkup_done: bool,
    medicine_done: bool,
    payment_done: bool,
}

impl Patient {
    fn new(name: &str) -> Self {
        Patient {
            name: name.to_string(),
            registeration_done: false,
            doctor_checkup_done: false,
            medicine_done: false,
            payment_done: false,
        }
    }
}

/// Handler interface
trait Department<'a> {
    fn execute(&self, patient: &mut Patient);
    fn set_next(&mut self, next: &'a dyn Department<'a>) -> &mut dyn Department<'a>;
}

/// Concrete handler
/// Step 1. patient arrives, they first get to Reception.
struct Reception<'a> {
    next: Option<&'a dyn Department<'a>>,
}

impl<'a> Department<'a> for Reception<'a> {
    fn execute(&self, patient: &mut Patient) {
        if patient.registeration_done {
            println!("Patient registration already done!");
            self.next.unwrap().execute(patient);
            return;
        }

        println!("Reception registrating patient...");
        patient.registeration_done = true;
        if let Some(next) = self.next {
            next.execute(patient);
        } else {
            println!("Chain of responsibility done here in Reception!");
        }
    }

    fn set_next(&mut self, next: &'a dyn Department<'a>) -> &mut dyn Department<'a> {
        self.next = Some(next);
        self
    }
}

/// Concrete Handler
/// Step 2. After reception, patient go to see the doctor.
struct Doctor<'a> {
    next: Option<&'a dyn Department<'a>>,
}

impl<'a> Department<'a> for Doctor<'a> {
    fn execute(&self, patient: &mut Patient) {
        if patient.doctor_checkup_done {
            println!("Doctor checkup already done!");
            self.next.unwrap().execute(patient);
            return;
        }

        println!("Doctor checking patient ...");
        patient.doctor_checkup_done = true;
        if let Some(next) = self.next {
            next.execute(patient);
        } else {
            println!("Chain of responsibility done here in Doctor!");
        }
    }

    fn set_next(&mut self, next: &'a dyn Department<'a>) -> &mut dyn Department<'a> {
        self.next = Some(next);
        self
    }
}

/// Concrete Handler
/// Step 3. After see the doctor, patient go to medical.
struct Medical<'a> {
    next: Option<&'a dyn Department<'a>>,
}

impl<'a> Department<'a> for Medical<'a> {
    fn execute(&self, patient: &mut Patient) {
        if patient.medicine_done {
            println!("Medicine already given to patient!");
            self.next.unwrap().execute(patient);
            return;
        }
        println!("Medical giving medicine to patient ...");
        patient.medicine_done = true;
        if let Some(next) = self.next {
            next.execute(patient);
        } else {
            println!("Chain of responsibility done here in Medical!");
        }
    }

    fn set_next(&mut self, next: &'a dyn Department<'a>) -> &mut dyn Department<'a> {
        self.next = Some(next);
        self
    }
}

/// Concrete Handler
/// Step 4. Last step, patient go to cashier to pay amount.
struct Cashier<'a> {
    next: Option<&'a dyn Department<'a>>,
}

impl<'a> Department<'a> for Cashier<'a> {
    fn execute(&self, patient: &mut Patient) {
        if patient.payment_done {
            println!("Payment already done!");
            return;
        }
        println!("Cashier getting money from patient ...");
        patient.payment_done = true;
    }

    fn set_next(&mut self, next: &'a dyn Department<'a>) -> &mut dyn Department<'a> {
        self.next = Some(next);
        self
    }
}

fn main() {
    let mut patient_jack = Patient::new("Jack");

    let mut doctor_alice = Doctor { next: None };
    let mut reception_north = Reception { next: None };

    reception_north.set_next(&mut doctor_alice);

    reception_north.execute(&mut patient_jack);
}
