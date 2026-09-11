#include "ansa_led.h"

/*
 * ANSA Status LED Configuration
 *
 * IMPORTANT:
 * The actual LED GPIO pin and active polarity must be
 * confirmed for the target board before hardware testing.
 *
 * Temporary development configuration:
 * GPIOC, Pin 13
 *
 * Active LOW:
 *   0 = LED ON
 *   1 = LED OFF
 */

#define ANSA_LED_GPIO_PORT       GPIOC
#define ANSA_LED_GPIO_PIN        13U

#define ANSA_LED_ACTIVE_LOW      1U

int ANSA_LED_Init(void)
{
    /*
     * Enable GPIOC clock.
     */
    RCC->AHB1ENR |= RCC_AHB1ENR_GPIOCEN;

    /*
     * Read back clock register to ensure the
     * peripheral clock is enabled.
     */
    (void)RCC->AHB1ENR;

    /*
     * Configure PC13 as general-purpose output.
     */
    ANSA_LED_GPIO_PORT->MODER &= ~GPIO_MODER_MODER13_Msk;
    ANSA_LED_GPIO_PORT->MODER |=
        (1U << GPIO_MODER_MODER13_Pos);

    /*
     * Push-pull output.
     */
    ANSA_LED_GPIO_PORT->OTYPER &= ~GPIO_OTYPER_OT13;

    /*
     * Low-speed output is sufficient for an LED.
     */
    ANSA_LED_GPIO_PORT->OSPEEDR &= ~GPIO_OSPEEDR_OSPEED13_Msk;

    /*
     * No pull-up / pull-down.
     */
    ANSA_LED_GPIO_PORT->PUPDR &= ~GPIO_PUPDR_PUPD13_Msk;

    /*
     * Start with LED OFF.
     */
    ANSA_LED_Off();

    return 0;
}

void ANSA_LED_On(void)
{
#if ANSA_LED_ACTIVE_LOW

    ANSA_LED_GPIO_PORT->BSRR =
        (1U << (ANSA_LED_GPIO_PIN + 16U));

#else

    ANSA_LED_GPIO_PORT->BSRR =
        (1U << ANSA_LED_GPIO_PIN);

#endif
}

void ANSA_LED_Off(void)
{
#if ANSA_LED_ACTIVE_LOW

    ANSA_LED_GPIO_PORT->BSRR =
        (1U << ANSA_LED_GPIO_PIN);

#else

    ANSA_LED_GPIO_PORT->BSRR =
        (1U << (ANSA_LED_GPIO_PIN + 16U));

#endif
}

void ANSA_LED_Toggle(void)
{
    ANSA_LED_GPIO_PORT->ODR ^=
        (1U << ANSA_LED_GPIO_PIN);
}
void ANSA_LED_Fault(void)
{
    ANSA_LED_Toggle();
}
