import React, { useState, useEffect } from 'react';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

interface FlashProgressPayload {
  transport: string;
  stage: string;
  progress: number;
  message: string;
  done: boolean;
  success: boolean;
}

export const FlashingProgress: React.FC = () => {
  const [progressData, setProgressData] = useState<FlashProgressPayload | null>(null);

  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    const setupListener = async () => {
      unlisten = await listen<FlashProgressPayload>('flash-progress', (event) => {
        setProgressData(event.payload);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  if (!progressData) return null;

  const { progress, message, done, success } = progressData;
  const isFinished = done;
  const hasFailed = done && !success;
  const hasSucceeded = done && success;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-6 font-sans bg-slate-950/85 backdrop-blur-md animate-in fade-in duration-300">
      <div className="w-full max-w-lg p-10 rounded-3xl bg-slate-900 border border-slate-700/80 shadow-2xl shadow-blue-900/10 flex flex-col items-center text-center">
        
        {/* Status Icon */}
        <div className="mb-8">
          {!isFinished && (
            <div className="relative flex items-center justify-center w-24 h-24 rounded-full bg-blue-500/10 border border-blue-500/30 shadow-[0_0_35px_rgba(59,130,246,0.3)]">
              <svg className="w-12 h-12 text-blue-400 animate-spin" fill="none" viewBox="0 0 24 24">
                <circle className="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="4"></circle>
                <path className="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
            </div>
          )}
          {hasSucceeded && (
            <div className="flex items-center justify-center w-24 h-24 rounded-full bg-emerald-500/10 border border-emerald-500/30 shadow-[0_0_35px_rgba(16,185,129,0.3)]">
              <svg className="w-12 h-12 text-emerald-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 13l4 4L19 7" />
              </svg>
            </div>
          )}
          {hasFailed && (
            <div className="flex items-center justify-center w-24 h-24 rounded-full bg-red-500/10 border border-red-500/30 shadow-[0_0_35px_rgba(239,68,68,0.3)]">
              <svg className="w-12 h-12 text-red-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
          )}
        </div>

        {/* Status Header */}
        <h2 className={`text-3xl font-bold tracking-tight mb-3 transition-colors duration-300 ${hasFailed ? 'text-red-400' : hasSucceeded ? 'text-emerald-400' : 'text-slate-50'}`}>
          {hasFailed ? 'Flashing Failed' : hasSucceeded ? 'Flashing Complete' : 'Flashing Firmware...'}
        </h2>
        <p className="text-sm font-medium text-slate-400 mb-8 min-h-[1.25rem]">
          {message}
        </p>

        {/* Progress Bar Container */}
        <div className="w-full">
          <div className="flex justify-between items-end mb-3">
            <span className="text-xs font-bold text-slate-500 uppercase tracking-wider">Progress</span>
            <span className={`text-lg font-bold transition-colors duration-300 ${hasFailed ? 'text-red-400' : hasSucceeded ? 'text-emerald-400' : 'text-blue-400'}`}>
              {progress}%
            </span>
          </div>
          
          <div className="h-4 w-full bg-slate-800 rounded-full overflow-hidden shadow-inner border border-slate-700/50">
            <div
              className={`h-full rounded-full transition-all duration-300 ease-out ${
                hasFailed 
                  ? 'bg-red-500 shadow-[0_0_15px_rgba(239,68,68,0.6)]' 
                  : hasSucceeded 
                    ? 'bg-emerald-500 shadow-[0_0_15px_rgba(16,185,129,0.6)]' 
                    : 'bg-gradient-to-r from-blue-600 to-blue-400 shadow-[0_0_20px_rgba(59,130,246,0.6)]'
              }`}
              style={{ width: `${Math.max(0, Math.min(100, progress))}%` }}
            />
          </div>
        </div>

        {/* Dismiss Button (Only show when done) */}
        {isFinished && (
          <button
            onClick={() => setProgressData(null)}
            className="mt-10 px-10 py-3 text-sm font-bold tracking-wide text-white bg-slate-800 rounded-xl border border-slate-700 hover:bg-slate-700 hover:border-slate-500 transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-slate-500/50"
          >
            CLOSE
          </button>
        )}
      </div>
    </div>
  );
};
