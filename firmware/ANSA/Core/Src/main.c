#include "stm32f411xe.h"

#include "../../Drivers/ANSA_BSP/Inc/ansa_clock.h"
#include "../../Drivers/ANSA_BSP/Inc/ansa_uart.h"
#include "../../Drivers/ANSA_BSP/Inc/ansa_led.h"
#include "../../Drivers/ANSA_BSP/Inc/ansa_info.h"
#include "../../Drivers/ANSA_BSP/Inc/ansa_time.h"

/**
 * @brief Main application entry point.
 *
 * Initializes:
 *  - System clock
 *  - ANSA system time base
 *  - ANSA status LED
 *  - ANSA UART
 *  - ANSA firmware information
 */
int main(void)
{
    /*
     * Initialize system clock.
     *
     * SYSCLK = 16 MHz HSI
     */
    ANSA_CLOCK_Init();

    /*
     * Initialize ANSA system time base.
     *
     * SysTick = 1 ms period
     */
    ANSA_TIME_Init();

    /*
     * Initialize ANSA status LED.
     */
    ANSA_LED_Init();

    /*
     * Initialize USART2.
     */
    ANSA_UART_Init();

    /*
     * Indicate that ANSA firmware has started.
     */
    ANSA_LED_On();

    /*
     * Send ANSA startup banner.
     */
    ANSA_UART_WriteString("ANSA\r\n");

    ANSA_UART_WriteString("BOARD: ");
    ANSA_UART_WriteString(ANSA_INFO_GetBoardName());
    ANSA_UART_WriteString("\r\n");

    ANSA_UART_WriteString("FW: ");
    ANSA_UART_WriteString(ANSA_INFO_GetFirmwareVersion());
    ANSA_UART_WriteString("\r\n");

    ANSA_UART_WriteString("BUILD: ");
    ANSA_UART_WriteString(ANSA_INFO_GetBuildType());
    ANSA_UART_WriteString("\r\n");

    ANSA_UART_WriteString("STATUS: READY\r\n");

    ANSA_LED_On();
    /*
     * Main application loop.
     */
    while (1)
    {
        static uint32_t last_toggle_ms = 0U;
        uint32_t current_ms;

        /*
         * Process incoming UART characters.
         */
        ANSA_UART_Process();

        /*
         * Non-blocking LED blink.
         */
        current_ms = ANSA_TIME_GetMilliseconds();

        if ((current_ms - last_toggle_ms) >= 500U)
        {
            last_toggle_ms = current_ms;
            ANSA_LED_Toggle();
        }
    }
}
