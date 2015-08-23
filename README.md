[![Build Status](https://travis-ci.org/mokus0/stm32.rs.svg)](https://travis-ci.org/mokus0/stm32.rs)


Low-level hardware descriptions of STM32 microcontroller register layouts.  Rather than using the SVD, which doesn't do much of anything to indicate commonalities between models or even peripherals within a single model, I'm just going through the reference manuals and defining types for each peripheral so they can be shared by compatible models (and thus, simplifying the construction of HALs on top of them)

Presently, I am targeting the following devices because they are the ones I have on hand (relevant ST document references in parens):

    STM32F0 (UM1690, RM0091)
    STM32F407 (UM1472, RM0090)
    STM32F429 (UM1670, RM0090)
    STM32L4 (UM1879, RM0351)

I'd be happy to add to this list in exchange for a dev board for your desired target :)

Note that these types are likely to be in a state of flux for a while.  If you like, you can drop me a note about any projects you have that might be using this library and I'll try to give a heads up or even a pull request when my changes might break your code.
