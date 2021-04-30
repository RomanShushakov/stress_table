import HidingContentButton from "./common_components/hiding-content-button.js";
customElements.define("hiding-content-button", HidingContentButton);

import FeaApp from "./components/fea-app.js";
import FeaAppTitleBar from "./components/fea-app-title-bar.js";
import FeaPreprocessor from "./components/fea-preprocessor.js";
import FeaPreprocessorMenu from "./components/fea-preprocessor-menu.js";
import FeaGeometry from "./components/fea-geometry.js";
import FeaProperties from "./components/fea-properties.js";
import FeaNode from "./components/fea-node.js";
import FeaElement from "./components/fea-element.js";
import FeaDisplacement from "./components/fea-displacement.js";
import FeaLoad from "./components/fea-load.js";
import FeaPreprocessorCanvas from "./components/fea-preprocessor-canvas.js";
import FeaPostprocessor from "./components/fea-postprocessor.js";
import FeaPostprocessorMenu from "./components/fea-postprocessor-menu.js";
import FeaPlotDisplacements from "./components/fea-plot-displacements.js";
import FeaPlotStresses from "./components/fea-plot-stresses.js";
import FeaPlotStrains from "./components/fea-plot-strains.js";
import FeaPlotForces from "./components/fea-plot-forces.js";
import FeaPostprocessorCanvas from "./components/fea-postprocessor-canvas.js";


customElements.define("fea-app", FeaApp);
customElements.define("fea-app-title-bar", FeaAppTitleBar);
customElements.define("fea-preprocessor", FeaPreprocessor);
customElements.define("fea-preprocessor-menu", FeaPreprocessorMenu);
customElements.define("fea-geometry", FeaGeometry);
customElements.define("fea-properties", FeaProperties);
customElements.define("fea-node", FeaNode);
customElements.define("fea-element", FeaElement);
customElements.define("fea-displacement", FeaDisplacement);
customElements.define("fea-load", FeaLoad);
customElements.define("fea-preprocessor-canvas", FeaPreprocessorCanvas);
customElements.define("fea-postprocessor", FeaPostprocessor);
customElements.define("fea-postprocessor-menu", FeaPostprocessorMenu);
customElements.define("fea-plot-displacements", FeaPlotDisplacements);
customElements.define("fea-plot-stresses", FeaPlotStresses);
customElements.define("fea-plot-strains", FeaPlotStrains);
customElements.define("fea-plot-forces", FeaPlotForces);
customElements.define("fea-postprocessor-canvas", FeaPostprocessorCanvas);


// import PreprocessorCanvas from "./components/preprocessor_canvas.js";
// import PostprocessorCanvas from "./components/postprocessor_canvas.js";


// customElements.define("preprocessor-canvas", PreprocessorCanvas);
// customElements.define("postprocessor-canvas", PostprocessorCanvas);