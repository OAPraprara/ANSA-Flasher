import React, { useState } from 'react';
import { DeviceStatusBar } from './components/DeviceStatusBar';
import { BoardPicker } from './components/BoardPicker';
import { VersionPicker } from './components/VersionPicker';
import { CustomizationForm } from './components/CustomizationForm';
import { FlashingProgress } from './components/FlashingProgress';
import { ConfirmationModal } from './components/ConfirmationModal';

export const App: React.FC = () => {
  const [showConfirm, setShowConfirm] = useState(false);

  return (
    <div className="min-h-screen bg-slate-950 p-8 pb-32 font-sans flex flex-col gap-8">
      <header className="text-center mb-8">
        <h1 className="text-4xl font-bold text-white mb-2 tracking-tight">ANSA Flasher</h1>
        <p className="text-slate-400">Firmware Deployment Tool</p>
      </header>

      <main className="max-w-4xl mx-auto w-full space-y-12">
        <section>
          <h2 className="text-xl font-semibold text-slate-200 mb-6 text-center">Select Target Board</h2>
          <BoardPicker />
        </section>

        <section className="flex flex-col items-center gap-6">
          <VersionPicker />
          <CustomizationForm />
        </section>

        <section className="flex justify-center mt-12">
          <button 
            onClick={() => setShowConfirm(true)}
            className="px-8 py-4 bg-blue-600 hover:bg-blue-500 text-white font-bold rounded-xl shadow-[0_0_20px_rgba(37,99,235,0.4)] hover:shadow-[0_0_30px_rgba(37,99,235,0.6)] transition-all text-lg tracking-wide focus:outline-none focus:ring-2 focus:ring-blue-500/50 active:scale-95"
          >
            FLASH DEVICE
          </button>
        </section>
      </main>

      <DeviceStatusBar />
      <FlashingProgress />
      <ConfirmationModal 
        isOpen={showConfirm} 
        onClose={() => setShowConfirm(false)} 
        onConfirm={() => setShowConfirm(false)} 
      />
    </div>
  );
};
