#[doc = "Register `bmx_cfg1` reader"]
pub struct R(crate::R<BMX_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BMX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BMX_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BMX_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `bmx_cfg1` writer"]
pub struct W(crate::W<BMX_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BMX_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<BMX_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BMX_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_bmx_berr_int_en` reader - "]
pub type REG_BMX_BERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_berr_int_en` writer - "]
pub type REG_BMX_BERR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_mcu_berr_int_en` reader - "]
pub type REG_MCU_BERR_INT_EN_R = crate::BitReader<bool>;
#[doc = "Field `reg_mcu_berr_int_en` writer - "]
pub type REG_MCU_BERR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reserved_2_15` reader - "]
pub type RESERVED_2_15_R = crate::FieldReader<u16, u16>;
#[doc = "Field `reg_bmx_qos_cpu` reader - "]
pub type REG_BMX_QOS_CPU_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_cpu` writer - "]
pub type REG_BMX_QOS_CPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_sdu` reader - "]
pub type REG_BMX_QOS_SDU_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_sdu` writer - "]
pub type REG_BMX_QOS_SDU_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_sec0` reader - "]
pub type REG_BMX_QOS_SEC0_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_sec0` writer - "]
pub type REG_BMX_QOS_SEC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_sec1` reader - "]
pub type REG_BMX_QOS_SEC1_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_sec1` writer - "]
pub type REG_BMX_QOS_SEC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_sec2` reader - "]
pub type REG_BMX_QOS_SEC2_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_sec2` writer - "]
pub type REG_BMX_QOS_SEC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_dma` reader - "]
pub type REG_BMX_QOS_DMA_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_dma` writer - "]
pub type REG_BMX_QOS_DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_cci` reader - "]
pub type REG_BMX_QOS_CCI_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_cci` writer - "]
pub type REG_BMX_QOS_CCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_pldma` reader - "]
pub type REG_BMX_QOS_PLDMA_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_pldma` writer - "]
pub type REG_BMX_QOS_PLDMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_blem` reader - "]
pub type REG_BMX_QOS_BLEM_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_blem` writer - "]
pub type REG_BMX_QOS_BLEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_emaca` reader - "]
pub type REG_BMX_QOS_EMACA_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_emaca` writer - "]
pub type REG_BMX_QOS_EMACA_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_dma2` reader - "]
pub type REG_BMX_QOS_DMA2_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_dma2` writer - "]
pub type REG_BMX_QOS_DMA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `reg_bmx_qos_sdhm` reader - "]
pub type REG_BMX_QOS_SDHM_R = crate::BitReader<bool>;
#[doc = "Field `reg_bmx_qos_sdhm` writer - "]
pub type REG_BMX_QOS_SDHM_W<'a, const O: u8> = crate::BitWriter<'a, u32, BMX_CFG1_SPEC, bool, O>;
#[doc = "Field `bmx_dbg_sel` reader - "]
pub type BMX_DBG_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bmx_dbg_sel` writer - "]
pub type BMX_DBG_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BMX_CFG1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_bmx_berr_int_en(&self) -> REG_BMX_BERR_INT_EN_R {
        REG_BMX_BERR_INT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_mcu_berr_int_en(&self) -> REG_MCU_BERR_INT_EN_R {
        REG_MCU_BERR_INT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15"]
    #[inline(always)]
    pub fn reserved_2_15(&self) -> RESERVED_2_15_R {
        RESERVED_2_15_R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn reg_bmx_qos_cpu(&self) -> REG_BMX_QOS_CPU_R {
        REG_BMX_QOS_CPU_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdu(&self) -> REG_BMX_QOS_SDU_R {
        REG_BMX_QOS_SDU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec0(&self) -> REG_BMX_QOS_SEC0_R {
        REG_BMX_QOS_SEC0_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec1(&self) -> REG_BMX_QOS_SEC1_R {
        REG_BMX_QOS_SEC1_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn reg_bmx_qos_sec2(&self) -> REG_BMX_QOS_SEC2_R {
        REG_BMX_QOS_SEC2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma(&self) -> REG_BMX_QOS_DMA_R {
        REG_BMX_QOS_DMA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn reg_bmx_qos_cci(&self) -> REG_BMX_QOS_CCI_R {
        REG_BMX_QOS_CCI_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reg_bmx_qos_pldma(&self) -> REG_BMX_QOS_PLDMA_R {
        REG_BMX_QOS_PLDMA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn reg_bmx_qos_blem(&self) -> REG_BMX_QOS_BLEM_R {
        REG_BMX_QOS_BLEM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn reg_bmx_qos_emaca(&self) -> REG_BMX_QOS_EMACA_R {
        REG_BMX_QOS_EMACA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn reg_bmx_qos_dma2(&self) -> REG_BMX_QOS_DMA2_R {
        REG_BMX_QOS_DMA2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn reg_bmx_qos_sdhm(&self) -> REG_BMX_QOS_SDHM_R {
        REG_BMX_QOS_SDHM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn bmx_dbg_sel(&self) -> BMX_DBG_SEL_R {
        BMX_DBG_SEL_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_berr_int_en(&mut self) -> REG_BMX_BERR_INT_EN_W<0> {
        REG_BMX_BERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_mcu_berr_int_en(&mut self) -> REG_MCU_BERR_INT_EN_W<1> {
        REG_MCU_BERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_cpu(&mut self) -> REG_BMX_QOS_CPU_W<16> {
        REG_BMX_QOS_CPU_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sdu(&mut self) -> REG_BMX_QOS_SDU_W<17> {
        REG_BMX_QOS_SDU_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec0(&mut self) -> REG_BMX_QOS_SEC0_W<18> {
        REG_BMX_QOS_SEC0_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec1(&mut self) -> REG_BMX_QOS_SEC1_W<19> {
        REG_BMX_QOS_SEC1_W::new(self)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sec2(&mut self) -> REG_BMX_QOS_SEC2_W<20> {
        REG_BMX_QOS_SEC2_W::new(self)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_dma(&mut self) -> REG_BMX_QOS_DMA_W<21> {
        REG_BMX_QOS_DMA_W::new(self)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_cci(&mut self) -> REG_BMX_QOS_CCI_W<22> {
        REG_BMX_QOS_CCI_W::new(self)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_pldma(&mut self) -> REG_BMX_QOS_PLDMA_W<23> {
        REG_BMX_QOS_PLDMA_W::new(self)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_blem(&mut self) -> REG_BMX_QOS_BLEM_W<24> {
        REG_BMX_QOS_BLEM_W::new(self)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_emaca(&mut self) -> REG_BMX_QOS_EMACA_W<25> {
        REG_BMX_QOS_EMACA_W::new(self)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_dma2(&mut self) -> REG_BMX_QOS_DMA2_W<26> {
        REG_BMX_QOS_DMA2_W::new(self)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    #[must_use]
    pub fn reg_bmx_qos_sdhm(&mut self) -> REG_BMX_QOS_SDHM_W<27> {
        REG_BMX_QOS_SDHM_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn bmx_dbg_sel(&mut self) -> BMX_DBG_SEL_W<28> {
        BMX_DBG_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "bmx_cfg1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmx_cfg1](index.html) module"]
pub struct BMX_CFG1_SPEC;
impl crate::RegisterSpec for BMX_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bmx_cfg1::R](R) reader structure"]
impl crate::Readable for BMX_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bmx_cfg1::W](W) writer structure"]
impl crate::Writable for BMX_CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets bmx_cfg1 to value 0"]
impl crate::Resettable for BMX_CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
