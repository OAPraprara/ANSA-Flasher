import React, { useState, useEffect } from 'react';

export interface CustomConfig {
  droneId?: number;
  ssid?: string;
  password?: string;
}

interface CustomizationFormProps {
  onChange: (config: CustomConfig) => void;
}

export const CustomizationForm: React.FC<CustomizationFormProps> = ({ onChange }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [droneId, setDroneId] = useState('');
  const [wifiSsid, setWifiSsid] = useState('');
  const [wifiPassword, setWifiPassword] = useState('');

  useEffect(() => {
    onChange({
      droneId: droneId ? parseInt(droneId, 10) : undefined,
      ssid: wifiSsid,
      password: wifiPassword,
    });
  }, [droneId, wifiSsid, wifiPassword, onChange]);

  return (
    <div className="w-full max-w-xl mx-auto font-sans">
      {/* Toggle Button */}
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className="flex items-center gap-2 text-sm font-medium text-slate-300 hover:text-white transition-colors duration-200 focus:outline-none"
      >
        <svg
          className={`w-5 h-5 transition-transform duration-300 ${isExpanded ? 'rotate-90 text-blue-400' : 'text-slate-500'}`}
          fill="none"
          viewBox="0 0 24 24"
          stroke="currentColor"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2.5} d="M9 5l7 7-7 7" />
        </svg>
        Advanced Firmware Configuration
      </button>

      {/* Collapsible Form Container */}
      <div
        className={`mt-4 overflow-hidden transition-all duration-300 ease-in-out ${
          isExpanded ? 'max-h-[500px] opacity-100' : 'max-h-0 opacity-0'
        }`}
      >
        <div className="p-6 rounded-2xl bg-slate-800/40 border border-slate-700/80 backdrop-blur-sm shadow-xl space-y-5">
          
          {/* Drone ID Field */}
          <div>
            <label htmlFor="droneId" className="block text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2 ml-1">
              Drone ID (Numeric)
            </label>
            <input
              id="droneId"
              type="number"
              placeholder="e.g., 1"
              value={droneId}
              onChange={(e) => setDroneId(e.target.value)}
              className="w-full px-4 py-3 bg-slate-900/60 border border-slate-700 rounded-xl text-slate-100 placeholder-slate-600 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25 transition-all duration-200"
            />
          </div>

          {/* WiFi SSID Field */}
          <div>
            <label htmlFor="wifiSsid" className="block text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2 ml-1">
              WiFi SSID
            </label>
            <input
              id="wifiSsid"
              type="text"
              placeholder="Your Network Name"
              value={wifiSsid}
              onChange={(e) => setWifiSsid(e.target.value)}
              className="w-full px-4 py-3 bg-slate-900/60 border border-slate-700 rounded-xl text-slate-100 placeholder-slate-600 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25 transition-all duration-200"
            />
          </div>

          {/* WiFi Password Field */}
          <div>
            <label htmlFor="wifiPassword" className="block text-xs font-semibold text-slate-400 uppercase tracking-wider mb-2 ml-1">
              WiFi Password
            </label>
            <input
              id="wifiPassword"
              type="password"
              placeholder="••••••••"
              value={wifiPassword}
              onChange={(e) => setWifiPassword(e.target.value)}
              className="w-full px-4 py-3 bg-slate-900/60 border border-slate-700 rounded-xl text-slate-100 placeholder-slate-600 focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500/25 transition-all duration-200"
            />
          </div>

        </div>
      </div>
    </div>
  );
};
