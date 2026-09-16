import React from 'react';

export interface SuccessScreenProps {
  boardName: string;
  version: string;
  transport: string;
  customConfig?: { droneId?: number; ssid?: string };
  onReset: () => void;
}

export const SuccessScreen: React.FC<SuccessScreenProps> = ({
  boardName,
  version,
  transport,
  customConfig,
  onReset,
}) => {
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-6 font-sans bg-slate-950/85 backdrop-blur-md animate-in fade-in zoom-in-95 duration-500">
      <div className="w-full max-w-xl p-10 rounded-3xl bg-slate-900 border border-slate-700/80 shadow-2xl shadow-emerald-900/10 flex flex-col items-center">
        
        {/* Celebration Icon */}
        <div className="mb-6 flex items-center justify-center w-24 h-24 rounded-full bg-emerald-500/10 border border-emerald-500/30 shadow-[0_0_40px_rgba(16,185,129,0.3)] animate-pulse">
          <svg className="w-12 h-12 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={3} d="M5 13l4 4L19 7" />
          </svg>
        </div>

        {/* Title */}
        <h2 className="text-3xl font-extrabold tracking-tight text-emerald-400 mb-8 text-center">
          Firmware Flashed Successfully!
        </h2>

        {/* Summary Card */}
        <div className="w-full p-6 mb-8 rounded-2xl bg-slate-950/60 border border-slate-800 shadow-inner">
          <h3 className="text-xs font-bold text-slate-500 uppercase tracking-widest mb-5 border-b border-slate-800 pb-3">
            Flashing Summary
          </h3>
          
          <div className="space-y-4">
            <div className="flex justify-between items-center">
              <span className="text-sm font-medium text-slate-400">Target Board</span>
              <span className="text-sm font-semibold text-slate-200">{boardName}</span>
            </div>
            
            <div className="flex justify-between items-center">
              <span className="text-sm font-medium text-slate-400">Firmware</span>
              <span className="text-sm font-semibold text-blue-400">{version}</span>
            </div>

            <div className="flex justify-between items-center">
              <span className="text-sm font-medium text-slate-400">Transport Method</span>
              <span className="text-sm font-semibold text-slate-200">{transport}</span>
            </div>

            {customConfig && (customConfig.droneId !== undefined || customConfig.ssid !== undefined) && (
              <>
                <div className="border-t border-slate-800/50 my-2 pt-2"></div>
                {customConfig.droneId !== undefined && (
                  <div className="flex justify-between items-center mt-3">
                    <span className="text-sm font-medium text-slate-400">Custom Drone ID</span>
                    <span className="text-sm font-semibold text-amber-400">{customConfig.droneId}</span>
                  </div>
                )}
                {customConfig.ssid && (
                  <div className="flex justify-between items-center mt-3">
                    <span className="text-sm font-medium text-slate-400">WiFi SSID</span>
                    <span className="text-sm font-semibold text-amber-400">{customConfig.ssid}</span>
                  </div>
                )}
              </>
            )}
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex flex-col w-full gap-4">
          <button
            onClick={onReset}
            className="w-full px-6 py-4 text-sm font-bold tracking-wide text-white bg-blue-600 rounded-xl shadow-[0_0_20px_rgba(37,99,235,0.4)] hover:bg-blue-500 hover:shadow-[0_0_25px_rgba(37,99,235,0.6)] active:scale-95 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-blue-500/50"
          >
            FLASH ANOTHER DEVICE
          </button>
          
          <button
            onClick={() => console.log('Safely disconnecting...')}
            className="w-full px-6 py-4 text-sm font-semibold text-slate-300 bg-slate-800 border border-slate-700 rounded-xl hover:bg-slate-700 hover:text-white transition-colors duration-200 focus:outline-none focus:ring-2 focus:ring-slate-500/50"
          >
            Safely Disconnect
          </button>
        </div>
        
      </div>
    </div>
  );
};
