import React, { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';
import { DeviceStatusBar } from './components/DeviceStatusBar';
import { BoardPicker } from './components/BoardPicker';
import { VersionPicker } from './components/VersionPicker';
import { CustomizationForm, CustomConfig } from './components/CustomizationForm';
import { FlashingProgress } from './components/FlashingProgress';
import { ConfirmationModal } from './components/ConfirmationModal';
import { SuccessScreen } from './components/SuccessScreen';
import { Manifest, Board, FirmwareVersion } from './types';

export const App: React.FC = () => {
  const [manifest, setManifest] = useState<Manifest | null>(null);
  const [selectedBoardId, setSelectedBoardId] = useState<string | null>(null);
  const [selectedVersion, setSelectedVersion] = useState<FirmwareVersion | null>(null);
  const [customConfig, setCustomConfig] = useState<CustomConfig>({});
  
  const [showConfirm, setShowConfirm] = useState(false);
  const [showSuccess, setShowSuccess] = useState(false);

  useEffect(() => {
    const fetchManifest = async () => {
      try {
        const res = await fetch("https://pub-204a9a1fa59a4769a5622b8b9fb81b14.r2.dev/manifest.json");
        const data = await res.json();
        setManifest(data);
      } catch (e) {
        console.error("Failed to fetch manifest from R2, falling back to local:", e);
        try {
          const res = await fetch("/manifest.json");
          const data = await res.json();
          setManifest(data);
        } catch (localErr) {
          console.error("Failed to load local manifest:", localErr);
        }
      }
    };
    fetchManifest();
  }, []);

  const selectedBoard = manifest?.boards.find(b => b.id === selectedBoardId);

  // If a board is selected but no version, or board changes, reset version or auto-select first
  useEffect(() => {
    if (selectedBoard && selectedBoard.versions.length > 0) {
      if (!selectedVersion || !selectedBoard.versions.find(v => v.version === selectedVersion.version)) {
        setSelectedVersion(selectedBoard.versions[0]);
      }
    } else {
      setSelectedVersion(null);
    }
  }, [selectedBoardId, selectedBoard]);

  const handleCustomConfigChange = useCallback((config: CustomConfig) => {
    setCustomConfig(config);
  }, []);

  const handleFlashConfirm = async () => {
    setShowConfirm(false);
    if (!selectedBoard || !selectedVersion) return;

    try {
      // Step 1: Download firmware
      const firmwarePath = await invoke<string>("download_firmware", { 
        url: selectedVersion.url, 
        outputPath: `temp_${selectedBoard.id}.bin` 
      });

      // Step 2: Verify SHA
      const isValid = await invoke<boolean>("verify_firmware_sha256", { 
        filePath: firmwarePath, 
        expectedHash: selectedVersion.sha256 
      });

      if (!isValid) {
        alert("Firmware verification failed! Hash mismatch.");
        return;
      }

      // Step 3: Flash based on transport
      if (selectedBoard.transport === "dfu") {
        await invoke("flash_firmware_dfu", { firmwarePath });
      } else {
        // default to swd/stlink
        await invoke("flash_firmware_stlink", { firmwarePath });
      }
      
      // Step 4: Show success screen
      setShowSuccess(true);
    } catch (err) {
      console.error("Flashing failed:", err);
      alert("Flashing failed: " + err);
    }
  };

  const resetFlow = () => {
    setShowSuccess(false);
    setSelectedBoardId(null);
    setSelectedVersion(null);
  };

  if (showSuccess && selectedBoard && selectedVersion) {
    return (
      <SuccessScreen 
        boardName={selectedBoard.name} 
        version={selectedVersion.version} 
        transport={selectedBoard.transport} 
        customConfig={customConfig} 
        onReset={resetFlow} 
      />
    );
  }

  return (
    <div className="min-h-screen bg-slate-950 p-8 pb-32 font-sans flex flex-col gap-8">
      <header className="text-center mb-8">
        <h1 className="text-4xl font-bold text-white mb-2 tracking-tight">ANSA Flasher</h1>
        <p className="text-slate-400">Firmware Deployment Tool</p>
      </header>

      <main className="max-w-4xl mx-auto w-full space-y-12">
        <section>
          <BoardPicker 
            boards={manifest?.boards || []} 
            selectedBoardId={selectedBoardId} 
            onSelectBoard={setSelectedBoardId} 
          />
        </section>

        <section className="flex flex-col items-center gap-6">
          <VersionPicker 
            versions={selectedBoard?.versions || []} 
            selectedVersion={selectedVersion} 
            onSelectVersion={setSelectedVersion} 
          />
          <CustomizationForm onChange={handleCustomConfigChange} />
        </section>

        <section className="flex justify-center mt-12">
          <button 
            onClick={() => setShowConfirm(true)}
            disabled={!selectedBoard || !selectedVersion}
            className={`px-8 py-4 font-bold rounded-xl shadow-[0_0_20px_rgba(37,99,235,0.4)] transition-all text-lg tracking-wide focus:outline-none focus:ring-2 focus:ring-blue-500/50 
              ${!selectedBoard || !selectedVersion 
                ? 'bg-slate-700 text-slate-400 cursor-not-allowed shadow-none' 
                : 'bg-blue-600 hover:bg-blue-500 text-white hover:shadow-[0_0_30px_rgba(37,99,235,0.6)] active:scale-95'}`}
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
        onConfirm={handleFlashConfirm} 
        board={selectedBoard}
        version={selectedVersion!}
      />
    </div>
  );
};
