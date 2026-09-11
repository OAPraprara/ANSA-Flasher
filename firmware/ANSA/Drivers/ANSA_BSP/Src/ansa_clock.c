#include "../../ANSA_BSP/Inc/ansa_clock.h"

/*
 * STM32F411 internal HSI oscillator:
 *
 * HSI = 16 MHz
 *
 * We are currently using HSI directly as SYSCLK.
 */
#define ANSA_CLOCK_HSI_FREQUENCY    16000000U

static uint32_t ansa_system_clock_frequency =
    ANSA_CLOCK_HSI_FREQUENCY;

int ANSA_CLOCK_Init(void)
{
    /*
     * 1. Enable HSI oscillator.
     */
    RCC->CR |= RCC_CR_HSION;

    /*
     * 2. Wait until HSI is ready.
     */
    while ((RCC->CR & RCC_CR_HSIRDY) == 0U)
    {
        /* Wait */
    }

    /*
     * 3. Select HSI as the system clock.
     *
     * SW = 00 -> HSI
     */
    RCC->CFGR &= ~RCC_CFGR_SW_Msk;
    RCC->CFGR |= RCC_CFGR_SW_HSI;

    /*
     * 4. Wait until HSI is confirmed as SYSCLK.
     *
     * SWS = 00 -> HSI
     */
    while ((RCC->CFGR & RCC_CFGR_SWS_Msk) != RCC_CFGR_SWS_HSI)
    {
        /* Wait */
    }

    /*
     * 5. Configure AHB prescaler.
     *
     * HPRE = 0000 -> SYSCLK not divided
     * HCLK = 16 MHz
     */
    RCC->CFGR &= ~RCC_CFGR_HPRE_Msk;

    /*
     * 6. Configure APB1 prescaler.
     *
     * PPRE1 = 000 -> HCLK not divided
     * PCLK1 = 16 MHz
     */
    RCC->CFGR &= ~RCC_CFGR_PPRE1_Msk;

    /*
     * 7. Configure APB2 prescaler.
     *
     * PPRE2 = 000 -> HCLK not divided
     * PCLK2 = 16 MHz
     */
    RCC->CFGR &= ~RCC_CFGR_PPRE2_Msk;

    /*
     * 8. Store system clock frequency.
     */
    ansa_system_clock_frequency =
        ANSA_CLOCK_HSI_FREQUENCY;

    return 0;
}

uint32_t ANSA_CLOCK_GetFrequency(void)
{
    return ansa_system_clock_frequency;
}
