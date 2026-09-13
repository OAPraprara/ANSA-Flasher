# ANSA Flasher - UART Fallback Guide

If the primary flashing methods—USB DFU or ST-Link (SWD)—are unavailable, the STM32 ROM bootloader supports a fallback mechanism. You can flash the firmware via a standard USB-to-Serial (UART) adapter using the ST AN3155 protocol.

## Wiring Instructions

To establish a physical connection between your USB-to-Serial adapter and the STM32 microcontroller, follow this physical hookup:

* **Adapter TX** to **STM32 RX** (typically **PA10**)
* **Adapter RX** to **STM32 TX** (typically **PA9**)
* **Adapter GND** to **STM32 GND**

## Boot Mode Configuration

To force the microcontroller to enter the UART bootloader instead of executing the main application, you must configure its boot pins before powering it on. 

Explicitly pull the **BOOT0 pin HIGH** (connect it to 3.3V) and reset or power-cycle the board. This signals the STM32 ROM to start the bootloader.

## Recommended Host Tool

The recommended open-source host tool for interacting with the ROM bootloader via this fallback method is **stm32flash**.
