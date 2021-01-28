var searchIndex = JSON.parse('{\
"imxrt_ccm":{"doc":"Clock Control Module (CCM) driver for i.MX RT systems","i":[[0,"arm","imxrt_ccm","ARM clock control",null,null],[3,"ARMClock","imxrt_ccm::arm","The ARM clock frequency",null,null],[12,"0","","",0,null],[3,"IPGClock","","The IPG clock frequency",null,null],[12,"0","","",1,null],[5,"set_frequency","","Set the ARM clock frequency, returning the ARM and IPG …",null,[[]]],[5,"frequency","","Returns the ARM and IPG clock frequencies",null,[[]]],[0,"i2c","imxrt_ccm","I2C clock control",null,null],[3,"I2CClock","imxrt_ccm::i2c","The I2C clock",null,null],[11,"configure_divider","","Configure the I2C clocks, and supply the clock divider.",2,[[]]],[11,"configure","","Configure the I2C clocks with a default divider",2,[[]]],[4,"I2C","","Peripheral instance identifier for I2C",null,null],[13,"I2C1","","",3,null],[13,"I2C2","","",3,null],[13,"I2C3","","",3,null],[13,"I2C4","","",3,null],[11,"set_clock_gate","","Set the clock gate setting for the I2C instance",2,[[["clockgate",4]]]],[11,"clock_gate","","Returns the clock gate setting for the I2C instance",2,[[],["clockgate",4]]],[11,"frequency","","Returns the configured I2C clock frequency",2,[[]]],[5,"configure","","Configure the I2C clock root, specifying a clock divider",null,[[]]],[5,"frequency","","Returns the I2C clock frequency",null,[[]]],[0,"perclock","imxrt_ccm","Periodic clock",null,null],[3,"PerClock","imxrt_ccm::perclock","The periodic clock root",null,null],[4,"GPT","","Peripheral instance identifier for GPT",null,null],[13,"GPT1","","",4,null],[13,"GPT2","","",4,null],[4,"Selection","","Periodic clock selection",null,null],[13,"IPG","","Use the IPG clock root",5,null],[13,"Oscillator","","Use the crystal oscillator",5,null],[3,"PIT","","Peripheral instance identifier for PIT",null,null],[11,"frequency","","Returns the configured periodic clock frequency",6,[[]]],[11,"try_frequency","","Try to read the periodic clock frequency, returning the …",6,[[],["option",4]]],[11,"selection","","Returns the periodic clock selection",6,[[],["selection",4]]],[11,"clock_gate_gpt","","Returns the clock gate setting for the GPT",6,[[],["clockgate",4]]],[11,"set_clock_gate_gpt","","Set the clock gate for the GPT",6,[[["clockgate",4]]]],[11,"clock_gate_pit","","Returns the clock gate setting for the PIT",6,[[],["clockgate",4]]],[11,"set_clock_gate_pit","","Set the clock gate for the PIT",6,[[["clockgate",4]]]],[11,"configure_selection_divider","","Configure the periodic clock root, specifying the clock …",6,[[["selection",4]]]],[11,"configure","","Configure the periodic clock root with a default divider. …",6,[[]]],[5,"configure","","Configure the periodic clock root",null,[[["selection",4]]]],[5,"frequency","","Returns the periodic clock frequency",null,[[]]],[5,"selection","","Returns the periodic clock selection",null,[[],["selection",4]]],[0,"spi","imxrt_ccm","SPI clock control",null,null],[3,"SPIClock","imxrt_ccm::spi","The SPI clock",null,null],[11,"configure_divider","","Configure the SPI clocks, specifying the clock divider",7,[[]]],[11,"configure","","Configure the SPI clocks with a default divider",7,[[]]],[4,"SPI","","Peripheral instance identifier for SPI",null,null],[13,"SPI1","","",8,null],[13,"SPI2","","",8,null],[13,"SPI3","","",8,null],[13,"SPI4","","",8,null],[11,"clock_gate","","Returns the clock gate setting for the SPI instance",7,[[],["clockgate",4]]],[11,"set_clock_gate","","Set the clock gate for the SPI instance",7,[[["clockgate",4]]]],[11,"frequency","","Returns the SPI clock frequency",7,[[]]],[5,"configure","","Configure the SPI clock root",null,[[]]],[5,"frequency","","Returns the SPI clock frequency",null,[[]]],[0,"uart","imxrt_ccm","UART clock control",null,null],[3,"UARTClock","imxrt_ccm::uart","The UART clock",null,null],[11,"configure","","Configure the UART clocks with default divider",9,[[]]],[11,"configure_divider","","Configure the UART clocks with a clock divider.",9,[[]]],[4,"UART","","Peripheral instance identifier for UART",null,null],[13,"UART1","","",10,null],[13,"UART2","","",10,null],[13,"UART3","","",10,null],[13,"UART4","","",10,null],[13,"UART5","","",10,null],[13,"UART6","","",10,null],[13,"UART7","","",10,null],[13,"UART8","","",10,null],[11,"clock_gate","","Returns the clock gate setting for the UART instance",9,[[],["clockgate",4]]],[11,"set_clock_gate","","Set the clock gate for the UART instance",9,[[["clockgate",4]]]],[11,"frequency","","Returns the UART clock frequency",9,[[]]],[5,"configure","","Configure the UART clock root",null,[[]]],[5,"frequency","","Returns the UART clock frequency",null,[[]]],[0,"ral","imxrt_ccm","Implementations for the imxrt-ral",null,null],[3,"Clocks","imxrt_ccm::ral","Pairs the RAL instances to CCM clocks",null,null],[6,"CCM","","Helper for a clock control module designed to the RAL …",null,null],[6,"PerClock","","A periodic clock that controls RAL PIT and GPT timings",null,null],[6,"UARTClock","","A UART clock that controls RAL LPUART timing",null,null],[6,"SPIClock","","A SPI clock that controls RAL LPSPI timing",null,null],[6,"I2CClock","","An I2C clock that contorls RAL LPI2C timing",null,null],[11,"from_ral","","Converts the <code>imxrt-ral</code> CCM instance into the <code>CCM</code> driver",11,[[["instance",3]]]],[3,"ClockGateLocation","imxrt_ccm","Describes the location of a clock gate field",null,null],[8,"ClockGateLocator","","A type that can locate a clock gate",null,null],[10,"location","","Returns the location of a clock gate",12,[[],["clockgatelocation",3]]],[8,"Instance","","A peripheral instance that has a clock gate",null,null],[16,"Inst","","An identifier that describes the instance",13,null],[10,"instance","","Returns the peripheral instance identifier",13,[[]]],[10,"is_valid","","Returns <code>true</code> if this instance is valid for a particular …",13,[[]]],[3,"DCDC","","Peripheral instance identifier for DCDC",null,null],[3,"DMA","","Peripheral instance identifier for DMA",null,null],[5,"set_clock_gate","","Set the clock gate for a peripheral instance",null,[[["clockgate",4]]]],[5,"get_clock_gate","","Returns the clock gate setting for a peripheral instance",null,[[],[["clockgate",4],["option",4]]]],[4,"ADC","","Peripheral instance identifier for ADCs",null,null],[13,"ADC1","","",14,null],[13,"ADC2","","",14,null],[4,"PWM","","Peripheral instance identifier for PWM",null,null],[13,"PWM1","","",15,null],[13,"PWM2","","",15,null],[13,"PWM3","","",15,null],[13,"PWM4","","",15,null],[8,"Clocks","","Correlates an instance type to a CCM clock root",null,null],[16,"PIT","","PIT instance",16,null],[16,"GPT","","GPT instance",16,null],[16,"UART","","UART instance",16,null],[16,"SPI","","SPI instance",16,null],[16,"I2C","","I2C instance",16,null],[3,"CCM","","The clock control module (CCM)",null,null],[11,"new","","Construct a new CCM peripheral",17,[[]]],[11,"clock_gate_dcdc","","Returns the clock gate setting for the DCDC buck converter",17,[[],["clockgate",4]]],[11,"set_clock_gate_dcdc","","Set the clock gate for the DCDC buck converter",17,[[["clockgate",4]]]],[11,"clock_gate_dma","","Returns the clock gate setting for the DMA controller",17,[[],["clockgate",4]]],[11,"set_clock_gate_dma","","Set the clock gate for the DMA controller",17,[[["clockgate",4]]]],[11,"clock_gate_adc","","Returns the clock gate setting for the ADC",17,[[],["clockgate",4]]],[11,"set_clock_gate_adc","","Set the clock gate for the ADC peripheral",17,[[["clockgate",4]]]],[11,"clock_gate_pwm","","Returns the clock gate setting for the ADC",17,[[],["clockgate",4]]],[11,"set_clock_gate_pwm","","Set the clock gate for the PWM peripheral",17,[[["clockgate",4]]]],[11,"set_frequency_arm","","Set the ARM clock frequency, returning the new ARM and …",17,[[]]],[11,"frequency_arm","","Returns the ARM and IPG clock frequencies",17,[[]]],[4,"ClockGate","","Describes a clock gate setting",null,null],[13,"Off","","Clock is off during all modes",18,null],[13,"OnlyRun","","Clock is on in run mode, but off in wait and stop modes",18,null],[13,"On","","Clock is on in all modes, except stop mode",18,null],[11,"perclock","","Returns a reference to the periodic clock",17,[[],["perclock",3]]],[11,"perclock_mut","","Returns a mutable reference to the periodic clock",17,[[],["perclock",3]]],[11,"i2c_clock","","Returns a reference to the I2C clock",17,[[],["i2cclock",3]]],[11,"i2c_clock_mut","","Returns a mutable reference to the I2C clock",17,[[],["i2cclock",3]]],[11,"spi_clock","","Returns a reference to the SPI clock",17,[[],["spiclock",3]]],[11,"spi_clock_mut","","Returns a mutable reference to the SPI clock",17,[[],["spiclock",3]]],[11,"uart_clock","","Returns a reference to the UART clock",17,[[],["uartclock",3]]],[11,"uart_clock_mut","","Returns a mutable reference to the uart clock",17,[[],["uartclock",3]]],[11,"from","imxrt_ccm::arm","",0,[[]]],[11,"borrow","","",0,[[]]],[11,"borrow_mut","","",0,[[]]],[11,"try_from","","",0,[[],["result",4]]],[11,"into","","",0,[[]]],[11,"try_into","","",0,[[],["result",4]]],[11,"type_id","","",0,[[],["typeid",3]]],[11,"from","","",1,[[]]],[11,"borrow","","",1,[[]]],[11,"borrow_mut","","",1,[[]]],[11,"try_from","","",1,[[],["result",4]]],[11,"into","","",1,[[]]],[11,"try_into","","",1,[[],["result",4]]],[11,"type_id","","",1,[[],["typeid",3]]],[11,"from","imxrt_ccm::i2c","",2,[[]]],[11,"borrow","","",2,[[]]],[11,"borrow_mut","","",2,[[]]],[11,"try_from","","",2,[[],["result",4]]],[11,"into","","",2,[[]]],[11,"try_into","","",2,[[],["result",4]]],[11,"type_id","","",2,[[],["typeid",3]]],[11,"from","","",3,[[]]],[11,"borrow","","",3,[[]]],[11,"borrow_mut","","",3,[[]]],[11,"try_from","","",3,[[],["result",4]]],[11,"into","","",3,[[]]],[11,"try_into","","",3,[[],["result",4]]],[11,"type_id","","",3,[[],["typeid",3]]],[11,"from","imxrt_ccm::perclock","",6,[[]]],[11,"borrow","","",6,[[]]],[11,"borrow_mut","","",6,[[]]],[11,"try_from","","",6,[[],["result",4]]],[11,"into","","",6,[[]]],[11,"try_into","","",6,[[],["result",4]]],[11,"type_id","","",6,[[],["typeid",3]]],[11,"from","","",4,[[]]],[11,"borrow","","",4,[[]]],[11,"borrow_mut","","",4,[[]]],[11,"try_from","","",4,[[],["result",4]]],[11,"into","","",4,[[]]],[11,"try_into","","",4,[[],["result",4]]],[11,"type_id","","",4,[[],["typeid",3]]],[11,"from","","",5,[[]]],[11,"borrow","","",5,[[]]],[11,"borrow_mut","","",5,[[]]],[11,"try_from","","",5,[[],["result",4]]],[11,"into","","",5,[[]]],[11,"try_into","","",5,[[],["result",4]]],[11,"type_id","","",5,[[],["typeid",3]]],[11,"from","","",19,[[]]],[11,"borrow","","",19,[[]]],[11,"borrow_mut","","",19,[[]]],[11,"try_from","","",19,[[],["result",4]]],[11,"into","","",19,[[]]],[11,"try_into","","",19,[[],["result",4]]],[11,"type_id","","",19,[[],["typeid",3]]],[11,"from","imxrt_ccm::spi","",7,[[]]],[11,"borrow","","",7,[[]]],[11,"borrow_mut","","",7,[[]]],[11,"try_from","","",7,[[],["result",4]]],[11,"into","","",7,[[]]],[11,"try_into","","",7,[[],["result",4]]],[11,"type_id","","",7,[[],["typeid",3]]],[11,"from","","",8,[[]]],[11,"borrow","","",8,[[]]],[11,"borrow_mut","","",8,[[]]],[11,"try_from","","",8,[[],["result",4]]],[11,"into","","",8,[[]]],[11,"try_into","","",8,[[],["result",4]]],[11,"type_id","","",8,[[],["typeid",3]]],[11,"from","imxrt_ccm::uart","",9,[[]]],[11,"borrow","","",9,[[]]],[11,"borrow_mut","","",9,[[]]],[11,"try_from","","",9,[[],["result",4]]],[11,"into","","",9,[[]]],[11,"try_into","","",9,[[],["result",4]]],[11,"type_id","","",9,[[],["typeid",3]]],[11,"from","","",10,[[]]],[11,"borrow","","",10,[[]]],[11,"borrow_mut","","",10,[[]]],[11,"try_from","","",10,[[],["result",4]]],[11,"into","","",10,[[]]],[11,"try_into","","",10,[[],["result",4]]],[11,"type_id","","",10,[[],["typeid",3]]],[11,"from","imxrt_ccm::ral","",20,[[]]],[11,"borrow","","",20,[[]]],[11,"borrow_mut","","",20,[[]]],[11,"try_from","","",20,[[],["result",4]]],[11,"into","","",20,[[]]],[11,"try_into","","",20,[[],["result",4]]],[11,"type_id","","",20,[[],["typeid",3]]],[11,"from","imxrt_ccm","",21,[[]]],[11,"borrow","","",21,[[]]],[11,"borrow_mut","","",21,[[]]],[11,"try_from","","",21,[[],["result",4]]],[11,"into","","",21,[[]]],[11,"try_into","","",21,[[],["result",4]]],[11,"type_id","","",21,[[],["typeid",3]]],[11,"from","","",22,[[]]],[11,"borrow","","",22,[[]]],[11,"borrow_mut","","",22,[[]]],[11,"try_from","","",22,[[],["result",4]]],[11,"into","","",22,[[]]],[11,"try_into","","",22,[[],["result",4]]],[11,"type_id","","",22,[[],["typeid",3]]],[11,"from","","",23,[[]]],[11,"borrow","","",23,[[]]],[11,"borrow_mut","","",23,[[]]],[11,"try_from","","",23,[[],["result",4]]],[11,"into","","",23,[[]]],[11,"try_into","","",23,[[],["result",4]]],[11,"type_id","","",23,[[],["typeid",3]]],[11,"from","","",14,[[]]],[11,"borrow","","",14,[[]]],[11,"borrow_mut","","",14,[[]]],[11,"try_from","","",14,[[],["result",4]]],[11,"into","","",14,[[]]],[11,"try_into","","",14,[[],["result",4]]],[11,"type_id","","",14,[[],["typeid",3]]],[11,"from","","",15,[[]]],[11,"borrow","","",15,[[]]],[11,"borrow_mut","","",15,[[]]],[11,"try_from","","",15,[[],["result",4]]],[11,"into","","",15,[[]]],[11,"try_into","","",15,[[],["result",4]]],[11,"type_id","","",15,[[],["typeid",3]]],[11,"from","","",17,[[]]],[11,"borrow","","",17,[[]]],[11,"borrow_mut","","",17,[[]]],[11,"try_from","","",17,[[],["result",4]]],[11,"into","","",17,[[]]],[11,"try_into","","",17,[[],["result",4]]],[11,"type_id","","",17,[[],["typeid",3]]],[11,"from","","",18,[[]]],[11,"borrow","","",18,[[]]],[11,"borrow_mut","","",18,[[]]],[11,"try_from","","",18,[[],["result",4]]],[11,"into","","",18,[[]]],[11,"try_into","","",18,[[],["result",4]]],[11,"type_id","","",18,[[],["typeid",3]]],[11,"location","imxrt_ccm::i2c","",3,[[],["clockgatelocation",3]]],[11,"location","imxrt_ccm::perclock","",4,[[],["clockgatelocation",3]]],[11,"location","","",19,[[],["clockgatelocation",3]]],[11,"location","imxrt_ccm::spi","",8,[[],["clockgatelocation",3]]],[11,"location","imxrt_ccm::uart","",10,[[],["clockgatelocation",3]]],[11,"location","imxrt_ccm","",22,[[],["clockgatelocation",3]]],[11,"location","","",23,[[],["clockgatelocation",3]]],[11,"location","","",14,[[],["clockgatelocation",3]]],[11,"location","","",15,[[],["clockgatelocation",3]]],[11,"fmt","imxrt_ccm::arm","",0,[[["formatter",3]],["result",6]]],[11,"fmt","","",1,[[["formatter",3]],["result",6]]],[11,"fmt","imxrt_ccm::i2c","",3,[[["formatter",3]],["result",6]]],[11,"fmt","imxrt_ccm::perclock","",4,[[["formatter",3]],["result",6]]],[11,"fmt","","",5,[[["formatter",3]],["result",6]]],[11,"fmt","","",19,[[["formatter",3]],["result",6]]],[11,"fmt","imxrt_ccm::spi","",8,[[["formatter",3]],["result",6]]],[11,"fmt","imxrt_ccm::uart","",10,[[["formatter",3]],["result",6]]],[11,"fmt","imxrt_ccm","",22,[[["formatter",3]],["result",6]]],[11,"fmt","","",23,[[["formatter",3]],["result",6]]],[11,"fmt","","",14,[[["formatter",3]],["result",6]]],[11,"fmt","","",15,[[["formatter",3]],["result",6]]],[11,"fmt","","",18,[[["formatter",3]],["result",6]]],[11,"eq","imxrt_ccm::arm","",0,[[["armclock",3]]]],[11,"ne","","",0,[[["armclock",3]]]],[11,"eq","","",1,[[["ipgclock",3]]]],[11,"ne","","",1,[[["ipgclock",3]]]],[11,"eq","imxrt_ccm::i2c","",3,[[["i2c",4]]]],[11,"eq","imxrt_ccm::perclock","",4,[[["gpt",4]]]],[11,"eq","","",5,[[["selection",4]]]],[11,"eq","","",19,[[["pit",3]]]],[11,"eq","imxrt_ccm::spi","",8,[[["spi",4]]]],[11,"eq","imxrt_ccm::uart","",10,[[["uart",4]]]],[11,"eq","imxrt_ccm","",22,[[["dcdc",3]]]],[11,"eq","","",23,[[["dma",3]]]],[11,"eq","","",14,[[["adc",4]]]],[11,"eq","","",15,[[["pwm",4]]]],[11,"eq","","",18,[[["clockgate",4]]]],[11,"clone","imxrt_ccm::arm","",0,[[],["armclock",3]]],[11,"clone","","",1,[[],["ipgclock",3]]],[11,"clone","imxrt_ccm::i2c","",3,[[],["i2c",4]]],[11,"clone","imxrt_ccm::perclock","",4,[[],["gpt",4]]],[11,"clone","","",5,[[],["selection",4]]],[11,"clone","","",19,[[],["pit",3]]],[11,"clone","imxrt_ccm::spi","",8,[[],["spi",4]]],[11,"clone","imxrt_ccm::uart","",10,[[],["uart",4]]],[11,"clone","imxrt_ccm","",21,[[],["clockgatelocation",3]]],[11,"clone","","",22,[[],["dcdc",3]]],[11,"clone","","",23,[[],["dma",3]]],[11,"clone","","",14,[[],["adc",4]]],[11,"clone","","",15,[[],["pwm",4]]],[11,"clone","","",18,[[],["clockgate",4]]],[11,"from_ral","","Converts the <code>imxrt-ral</code> CCM instance into the <code>CCM</code> driver",17,[[["instance",3]]]]],"p":[[3,"ARMClock"],[3,"IPGClock"],[3,"I2CClock"],[4,"I2C"],[4,"GPT"],[4,"Selection"],[3,"PerClock"],[3,"SPIClock"],[4,"SPI"],[3,"UARTClock"],[4,"UART"],[6,"CCM"],[8,"ClockGateLocator"],[8,"Instance"],[4,"ADC"],[4,"PWM"],[8,"Clocks"],[3,"CCM"],[4,"ClockGate"],[3,"PIT"],[3,"Clocks"],[3,"ClockGateLocation"],[3,"DCDC"],[3,"DMA"]]}\
}');
addSearchOptions(searchIndex);initSearch(searchIndex);