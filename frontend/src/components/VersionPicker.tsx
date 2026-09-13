import React, { useState, useRef, useEffect } from 'react';

type ReleaseChannel = 'stable' | 'beta';

interface FirmwareVersion {
  version: string;
  channel: ReleaseChannel;
  url: string;
  sha256: string;
  signature_url: string;
  size_bytes: number;
}

const MOCK_VERSIONS: FirmwareVersion[] = [
  {
    version: "1.2.0",
    channel: "stable",
    url: "https://ansa.dev/firmware/f446re/ansa-1.2.0.bin",
    sha256: "a1b2c3...",
    signature_url: "https://ansa.dev/firmware/f446re/ansa-1.2.0.bin.sig",
    size_bytes: 131072
  },
  {
    version: "1.3.0-beta",
    channel: "beta",
    url: "https://ansa.dev/firmware/f446re/ansa-1.3.0-beta.bin",
    sha256: "d4e5f6...",
    signature_url: "https://ansa.dev/firmware/f446re/ansa-1.3.0-beta.bin.sig",
    size_bytes: 135168
  }
];

export const VersionPicker: React.FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [selected, setSelected] = useState<FirmwareVersion>(MOCK_VERSIONS[0]);
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
        setIsOpen(false);
      }
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const getChannelStyle = (channel: ReleaseChannel) => {
    return channel === 'stable'
      ? 'bg-emerald-500/10 border-emerald-500/30 text-emerald-400'
      : 'bg-amber-500/10 border-amber-500/30 text-amber-400';
  };

  return (
    <div className="w-full max-w-sm font-sans" ref={dropdownRef}>
      <label className="block text-sm font-medium text-slate-400 mb-2 pl-1">Firmware Version</label>
      
      <div className="relative">
        {/* Dropdown Toggle Button */}
        <button
          type="button"
          onClick={() => setIsOpen(!isOpen)}
          className={`
            w-full flex items-center justify-between px-4 py-3 text-left rounded-xl
            bg-slate-800/80 border backdrop-blur-sm transition-all duration-300 ease-out focus:outline-none
            ${isOpen 
              ? 'border-blue-500 shadow-[0_0_20px_rgba(59,130,246,0.25)] bg-slate-800' 
              : 'border-slate-700 hover:border-slate-500 hover:bg-slate-800'}
          `}
        >
          <div className="flex items-center gap-3">
            <span className="text-lg font-semibold text-slate-100">{selected.version}</span>
            <span className={`px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-wider border rounded-lg ${getChannelStyle(selected.channel)}`}>
              {selected.channel}
            </span>
          </div>
          
          <svg
            className={`w-5 h-5 text-slate-400 transition-transform duration-300 ${isOpen ? 'rotate-180' : ''}`}
            fill="none" viewBox="0 0 24 24" stroke="currentColor"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
          </svg>
        </button>

        {/* Dropdown Menu */}
        {isOpen && (
          <div className="absolute z-10 w-full mt-2 origin-top-right rounded-xl bg-slate-800 border border-slate-700 shadow-2xl overflow-hidden animate-in fade-in slide-in-from-top-2 duration-200">
            <div className="py-1">
              {MOCK_VERSIONS.map((v) => {
                const isSelected = selected.version === v.version;
                return (
                  <button
                    key={v.version}
                    onClick={() => {
                      setSelected(v);
                      setIsOpen(false);
                    }}
                    className={`
                      w-full flex items-center justify-between px-4 py-3 text-left transition-colors duration-200
                      ${isSelected ? 'bg-blue-500/15' : 'hover:bg-slate-700/60'}
                    `}
                  >
                    <div className="flex items-center gap-3">
                      <span className={`text-base font-semibold ${isSelected ? 'text-blue-400' : 'text-slate-200'}`}>
                        {v.version}
                      </span>
                      <span className={`px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-wider border rounded-lg ${getChannelStyle(v.channel)}`}>
                        {v.channel}
                      </span>
                    </div>
                    {isSelected && (
                      <svg className="w-5 h-5 text-blue-500" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M5 13l4 4L19 7" />
                      </svg>
                    )}
                  </button>
                );
              })}
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
