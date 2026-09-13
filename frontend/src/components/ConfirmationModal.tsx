import React from 'react';

interface ConfirmationModalProps {
  isOpen: boolean;
  onClose: () => void;
  onConfirm: () => void;
}

export const ConfirmationModal: React.FC<ConfirmationModalProps> = ({
  isOpen,
  onClose,
  onConfirm,
}) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 font-sans bg-slate-950/70 backdrop-blur-md animate-in fade-in duration-300">
      <div className="relative w-full max-w-lg p-8 rounded-2xl bg-slate-900 border border-slate-700 shadow-2xl shadow-red-900/10">
        
        {/* Warning Icon & Header */}
        <div className="flex flex-col items-center text-center mb-6">
          <div className="flex items-center justify-center w-16 h-16 rounded-full bg-amber-500/10 mb-5 border border-amber-500/20 shadow-[0_0_15px_rgba(245,158,11,0.2)]">
            <svg
              className="w-8 h-8 text-amber-500"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2.5}
                d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
              />
            </svg>
          </div>
          <h2 className="text-2xl font-bold tracking-tight text-slate-50 mb-2">Are you absolutely sure?</h2>
          <p className="text-slate-400">
            Proceeding will <strong className="text-red-400 font-semibold">completely erase</strong> the existing firmware on the connected STM32 device. This action cannot be undone.
          </p>
        </div>

        {/* Configuration Summary */}
        <div className="p-5 mb-8 rounded-xl bg-slate-950/50 border border-slate-800">
          <h3 className="text-xs font-bold text-slate-500 uppercase tracking-wider mb-4">Flashing Configuration</h3>
          
          <div className="flex justify-between items-center mb-3">
            <span className="text-sm font-medium text-slate-400">Target Board</span>
            <span className="text-sm font-semibold text-slate-200">ANSA OS Nucleo-F446RE</span>
          </div>
          
          <div className="flex justify-between items-center">
            <span className="text-sm font-medium text-slate-400">Firmware</span>
            <div className="flex items-center gap-2">
              <span className="text-sm font-semibold text-blue-400">Version 1.2.0</span>
              <span className="px-2 py-0.5 text-[10px] font-bold uppercase tracking-wider text-emerald-400 bg-emerald-500/10 border border-emerald-500/30 rounded-md">
                Stable
              </span>
            </div>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="flex gap-4">
          <button
            onClick={onClose}
            className="flex-1 px-4 py-3 text-sm font-semibold text-slate-300 bg-slate-800 border border-slate-700 rounded-xl hover:bg-slate-700 hover:text-white hover:border-slate-500 transition-all duration-200"
          >
            Cancel
          </button>
          <button
            onClick={onConfirm}
            className="flex-1 px-4 py-3 text-sm font-bold tracking-wide text-white bg-red-600 rounded-xl shadow-[0_0_20px_rgba(220,38,38,0.4)] hover:bg-red-500 hover:shadow-[0_0_25px_rgba(220,38,38,0.6)] active:scale-95 transition-all duration-200"
          >
            ERASE & FLASH
          </button>
        </div>
        
      </div>
    </div>
  );
};
