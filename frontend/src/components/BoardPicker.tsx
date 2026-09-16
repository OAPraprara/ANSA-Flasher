import React from 'react';
import { Board } from '../types';

interface BoardPickerProps {
  boards: Board[];
  selectedBoardId: string | null;
  onSelectBoard: (id: string) => void;
}

export const BoardPicker: React.FC<BoardPickerProps> = ({ boards, selectedBoardId, onSelectBoard }) => {
  return (
    <div className="w-full max-w-6xl mx-auto p-8 font-sans text-slate-100">
      <div className="mb-8">
        <h2 className="text-3xl font-bold tracking-tight text-white mb-2">Target Hardware</h2>
        <p className="text-slate-400">Select the board you wish to flash firmware to.</p>
      </div>
      
      {/* Responsive CSS Grid */}
      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
        {boards.map((board) => {
          const isSelected = selectedBoardId === board.id;
          
          return (
            <div
              key={board.id}
              onClick={() => onSelectBoard(board.id)}
              className={`
                group relative p-6 rounded-2xl cursor-pointer bg-slate-800/60 backdrop-blur-sm border transition-all duration-300 ease-out
                hover:-translate-y-1.5 hover:shadow-[0_12px_30px_-10px_rgba(59,130,246,0.3)] hover:border-slate-500
                ${isSelected 
                  ? 'border-blue-500 shadow-[0_0_20px_rgba(59,130,246,0.25)] bg-slate-800' 
                  : 'border-slate-700/80'}
              `}
            >
              {/* Active Selection Indicator */}
              {isSelected && (
                <div className="absolute top-4 right-4 w-3 h-3 bg-blue-500 rounded-full shadow-[0_0_8px_rgba(59,130,246,0.8)]" />
              )}
              
              <h3 className="text-xl font-semibold mb-4 text-slate-50">{board.name}</h3>
              
              {/* Badge Indicators */}
              <div className="flex flex-wrap gap-2 mt-auto pt-2">
                <span className="px-3 py-1 text-xs font-medium bg-slate-900 border border-slate-700 text-slate-300 rounded-lg">
                  {board.mcu}
                </span>
                <span className="px-3 py-1 text-xs font-medium bg-blue-500/10 border border-blue-500/30 text-blue-400 rounded-lg uppercase tracking-wider">
                  {board.transport}
                </span>
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
};
