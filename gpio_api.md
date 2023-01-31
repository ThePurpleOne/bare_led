# GPIO API
This is a really simple API to setup and use GPIO as outputs (for now) without having to calculate the registers and .

## TLDR
Basic blink:

```rust
pub extern "C" fn _start() -> ! 
{
	loop 
	{
		let mut pin = GPIO::new(2, PinMode::OUTPUT);
		pin.on();
		wait();
		pin.off();
		wait();
	}
}
```

## PINSEL
In order to select the PIN mode (Input or Output) we'll need to find the right address and set the bits.

### Registers addresses
As said at page 90 of the [BCM2837 datasheet](https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf):


The pin selection is done on the GPFSEL0-5 32bits Registers.

| Address     	| Function 	| Register size 	| Mode 	|
|-------------	|----------	|---------------	|------	|
| 0x3F20'0000 	| GPFSEL0  	| 32            	| R/W  	|
| 0x3F20'0004 	| GPFSEL1  	| 32            	| R/W  	|
| 0x3F20'0008 	| GPFSEL2  	| 32            	| R/W  	|
| 0x3F20'000C 	| GPFSEL3  	| 32            	| R/W  	|
| 0x3F20'0010 	| GPFSEL4  	| 32            	| R/W  	|
| 0x3F20'0014 	| GPFSEL5  	| 32            	| R/W  	|

Each of these 6 registers addresses 10 PINs (and only 4 for the last one):

- `GPFSEL0` : 0 to 9
- `GPFSEL1` : 10 to 19
- `GPFSEL2` : 20 to 29
- `GPFSEL3` : 30 to 39
- `GPFSEL4` : 40 to 49
- `GPFSEL5` : 50 to 53


### Registers organization
![](docs\images\FSEL\pin_bits_fsel0.png)
![](docs\images\FSEL\pin_bits_fsel1.png)
![](docs\images\FSEL\pin_bits_fsel2.png)
![](docs\images\FSEL\pin_bits_fsel3.png)
![](docs\images\FSEL\pin_bits_fsel4.png)
![](docs\images\FSEL\pin_bits_fsel5.png)

### Modes
Now that we know how to address each register, we need to know what value to put into it. We can put values in chunks of 3 bits data in this format:

- `000` = GPIO is an input
- `001` = GPIO is an output
- `100` = GPIO takes alternate function 0
- `101` = GPIO takes alternate function 1
- `110` = GPIO takes alternate function 2
- `111` = GPIO takes alternate function 3
- `011` = GPIO takes alternate function 4
- `010` = GPIO takes alternate function 5

But we'll only touch the first and second modes, **Input** and **Output**


### Adress a single GPIO
In order to address and set a single GPIO to set it as input or output, we need to find a way to select the right FSEL with only the pin number.

#### Get FSEL number

```bash
fsel_number = pin_number / 10;
```

The number 10 being the number of chunks used in each register (10 chunks of 3 bits)

#### Get the FSEL Chunk
Get the chunk in the FSEL register (0 to 9):

```bash
chunk_number = pin_number mod 30
```

The number 30 being the number of bits used in each register.


#### Example
For example if i chose the GPIO 32, i'll have:

```bash
fsel_number = 32 / 10 = 3
chunk_number = 32 mod 30 = 2
```

We thus need to address the FSEL3 and set the bits of the 2nd chunks.

![](docs\images\FSEL\pin_bits_fsel3_example.png)

We can get the final register address by adding the size of a register (4 bytes) times the **fsel_number** to the address of the FSEL0 (because the registers are contiguous)

```rust
pub fn new(pin : u32, mode : PinMode) -> Self
{
	if pin > 53 {panic!("Undefined pin number")};

	let chunk_nb = pin % 30; // 2 % 30 = 2
	let fsel_nb  = pin / 10; // 2 / 10 = 0
	// 
	let fsel_add =  GPIO_FSEL_BASE + (GPIO_REG_SIZE * fsel_nb);

	// Read the old value to avoid changing it
	let mut val : u32;
	
	unsafe
	{
		val = core::ptr::read_volatile(fsel_add as *mut u32);
	}
	
	val &= !0b111 << (chunk_nb * GPIO_CHUNK_SIZE); // Clear the 3 bits
	if mode == PinMode::OUTPUT
	{
		val |= GPIO_AS_OUTPUT << (chunk_nb * GPIO_CHUNK_SIZE);
	}

	// Write it back
	unsafe
	{
		core::ptr::write_volatile(fsel_add as *mut u32, val);
	}

	return GPIO{	pin:pin,
					mode:mode,
					state:PinState::OFF};
}
```

## Turn an output On of Off

Now that the GPIO is setup, we can turn it on by setting one of the 2 GPSET registers

### Registers addresses

| Address     	| Function 	| Register size 	| Mode 	|
|-------------	|----------	|---------------	|------	|
| 0x3F20'001C 	| SET0     	| 32            	| W    	|
| 0x3F20'0020 	| SET1     	| 32            	| W    	|
| 0x3F20'0028 	| CLEAR0    | 32            	| W    	|
| 0x3F20'002C 	| CLEAR1    | 32            	| W    	|

### Address a single GPIO

- SET0 register can turn GPIO 00 to 31
- SET1 register can turn GPIO 32 to 53
- CLEAR0 register can turn GPIO 00 to 31
- CLEAR1 register can turn GPIO 32 to 53

As said in the Datasheet:

> writing a “0” to the field has no effect.

This means we do not need to mask anything, we just need to put a 1 on the bit of the GPIO we want to turn On or Off

### Implementation

```rust
pub fn on(&mut self)
{
	let reg_nb   = self.pin / 32;
	let shift    = self.pin % 32;
	let reg_addr = GPIO_SET_BASE + (GPIO_REG_SIZE * reg_nb);
	unsafe
	{
		core::ptr::write_volatile(reg_addr as *mut u32, 1 << shift);
	}
}

pub fn off(&mut self)
{
	let reg_nb   = self.pin / 32;
	let shift    = self.pin % 32;
	let reg_addr = GPIO_CLR_BASE + (GPIO_REG_SIZE * reg_nb);
	unsafe
	{
		core::ptr::write_volatile(reg_addr as *mut u32, 1 << shift);
	}
}
```


## Setting Pullup / Pulldown
The setup of GPIO Pulldown or pullup is done in 2 steps.

- Load the `GPPUD` register with Pullup or Pulldown corresponding value
- Write to the `GPPUDCLK 0/1` registers to accept the selected setting in GPPUD into the clocked pins.

### GPIO Pull-up/down Register (GPPUD) 
The register only uses 2 bits to select the Pull mode:

- `00` : Neither, Off – disable pull-up/down
- `01` : Pulldown, Enable Pull Down control
- `10` : Pullup, Enable Pull Up control
- `11` : Reserved 

### GPIO Pull-up/down Clock Registers (GPPUDCLK 0/1)
In this register we need to select the bit to apply the setting (from GPPUD) on.

- PUDCLK0 register apply it on GPIO 00 to 31
- PUDCLK0 register apply it on GPIO 32 to 53


In order to fully apply the setting we need to:

1. Write the setting to GPPUD
2. Wait 150 cycles
3. Write to GPPUDCLK0/1 to clock the control signal into the GPIO pads
4. Wait 150 cycles
5. Write 0 to clear GPPUD
6. Write 0 to GPPUDCLK0/1 to remove the clock