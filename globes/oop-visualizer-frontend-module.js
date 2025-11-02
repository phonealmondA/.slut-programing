const { ContainerModule } = require('@theia/core/shared/inversify');
const { OOPVisualizerContribution } = require('./oop-visualizer-contribution');
const { bindViewContribution, FrontendApplicationContribution, WidgetFactory } = require('@theia/core/lib/browser');
const { OOPVisualizerWidget } = require('./oop-visualizer-widget');

module.exports = new ContainerModule(bind => {
    bindViewContribution(bind, OOPVisualizerContribution);
    bind(FrontendApplicationContribution).toService(OOPVisualizerContribution);
    bind(OOPVisualizerWidget).toSelf();
    bind(WidgetFactory).toDynamicValue(ctx => ({
        id: OOPVisualizerWidget.ID,
        createWidget: () => ctx.container.get(OOPVisualizerWidget)
    })).inSingletonScope();
});