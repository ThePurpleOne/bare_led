# GPIO API

## PINSEL
In order to select the PIN mode (Input or Output) we'll need to find the right address and set the bits.

### Registers addresses
As said at page 90 of the [BCM2837 datasheet](https://cs140e.sergio.bz/docs/BCM2837-ARM-Peripherals.pdf):


The pin selection is done on the GPFSEL0-5 32bits Registers.

| Address     	| Function 	| Register size 	| Mode 	|
|-------------	|----------	|---------------	|------	|
| 0x7E20'0000 	| GPFSEL0  	| 32            	| R/W  	|
| 0x7E20'0004 	| GPFSEL1  	| 32            	| R/W  	|
| 0x7E20'0008 	| GPFSEL2  	| 32            	| R/W  	|
| 0x7E20'000C 	| GPFSEL3  	| 32            	| R/W  	|
| 0x7E20'0010 	| GPFSEL4  	| 32            	| R/W  	|
| 0x7E20'0014 	| GPFSEL5  	| 32            	| R/W  	|

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
let chunk_nb = pin % 30;
let fsel_nb  = pin / 10;
let fsel_add =  GPIO_FSEL_BASE + (GPIO_REG_SIZE * fsel_reg);
```
