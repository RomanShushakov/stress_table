mod auxiliary_structs;
pub mod gl_aux_functions;
pub mod gl_aux_structs;

pub use auxiliary_structs::
    {
        View, DrawnBCData, NormalizedNode,
        FEDrawnNodeData, FEDrawnElementData,
        DrawnDisplacementInputOption, ResultView, MinMaxValues, // AnalysisResult, DrawnNode
    };
