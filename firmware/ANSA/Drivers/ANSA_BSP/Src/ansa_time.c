
#include "ansa_time.h"
#include "ansa_clock.h"
#include "stm32f411xe.h"

static volatile uint32_t ansa_time_ms = 0U;

int ANSA_TIME_Init(void)
{
    uint32_t system_clock_hz;

    system_clock_hz = ANSA_CLOCK_GetFrequency();

    if (system_clock_hz == 0U)
    {
        return -1;
    }

    /*
     * Configure SysTick for a 1 ms period.
     *
     * SysTick clock = processor clock
     * Reload = SYSCLK / 1000 - 1
     */
    SysTick->LOAD = (system_clock_hz / 1000U) - 1U;

    SysTick->VAL = 0U;

    SysTick->CTRL =
        SysTick_CTRL_CLKSOURCE_Msk |
        SysTick_CTRL_TICKINT_Msk |
        SysTick_CTRL_ENABLE_Msk;

    ansa_time_ms = 0U;

    return 0;
}

uint32_t ANSA_TIME_GetMilliseconds(void)
{
    return ansa_time_ms;
}

void ANSA_TIME_Tick(void)
{
    ansa_time_ms++;
}
