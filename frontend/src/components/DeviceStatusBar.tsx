import React, { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

type DeviceStatus = 'disconnected' | 'stlink' | 'dfu';

export const DeviceStatusBar: React.FC = () => {
  const [status, setStatus] = useState<DeviceStatus>('disconnected');

  useEffect(() => {
    // Listen for backend events (e.g., from Rust) regarding USB status
    const setupListener = async () => {
      const unlisten = await listen<string>('usb-status', (event) => {
        const payload = event.payload.toLowerCase();
        if (payload.includes('st-link') || payload.includes('swd')) {
          setStatus('stlink');
        } else if (payload.includes('dfu') || payload.includes('bootloader')) {
          setStatus('dfu');
        } else {
          setStatus('disconnected');
        }
      });

      return unlisten;
    };

    let unlistenFn: (() => void) | undefined;
    setupListener().then((fn) => {
      unlistenFn = fn;
    });

    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, []);

  const cycleMockStatus = () => {
    setStatus((prev) => {
      if (prev === 'disconnected') return 'stlink';
      if (prev === 'stlink') return 'dfu';
      return 'disconnected';
    });
  };

  const getStatusConfig = () => {
    switch (status) {
      case 'stlink':
        return {
          text: 'ST-Link (SWD) Connected',
          indicatorStyle: 'bg-green-500 shadow-[0_0_10px_rgba(34,197,94,0.8)]',
          textStyle: 'text-green-400',
        };
      case 'dfu':
        return {
          text: 'STM32 BOOTLOADER (DFU) Connected',
          indicatorStyle: 'bg-blue-500 shadow-[0_0_10px_rgba(59,130,246,0.8)]',
          textStyle: 'text-blue-400', // could also use amber-500 if preferred
        };
      case 'disconnected':
      default:
        return {
          text: 'No Device Detected',
          indicatorStyle: 'bg-red-500 shadow-[0_0_10px_rgba(239,68,68,0.8)]',
          textStyle: 'text-red-400',
        };
    }
  };

  const config = getStatusConfig();

  return (
    <div className="fixed bottom-0 left-0 w-full h-12 flex items-center justify-between px-6 bg-slate-900 border-t border-slate-700 shadow-[0_-4px_15px_rgba(0,0,0,0.3)] z-50 font-sans">
      
      {/* Status Display */}
      <div className="flex items-center gap-3">
        <div className={`w-3 h-3 rounded-full transition-all duration-500 ${config.indicatorStyle}`} />
        <span className={`text-sm font-semibold tracking-wide transition-colors duration-500 ${config.textStyle}`}>
          {config.text}
        </span>
      </div>

      {/* Mock Toggle - Visible in Development Mode Only */}
      {process.env.NODE_ENV === 'development' && (
        <button
          onClick={cycleMockStatus}
          className="px-3 py-1.5 text-xs font-medium bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-600 rounded transition-all active:scale-95"
          title="Cycle through hardware states manually"
        >
          Test Hardware States
        </button>
      )}
    </div>
  );
};
