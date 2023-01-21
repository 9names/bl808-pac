#[doc = "Register `codec_qos_ctrl` reader"]
pub struct R(crate::R<CODEC_QOS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CODEC_QOS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CODEC_QOS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CODEC_QOS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `codec_qos_ctrl` writer"]
pub struct W(crate::W<CODEC_QOS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CODEC_QOS_CTRL_SPEC>;
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
impl From<crate::W<CODEC_QOS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CODEC_QOS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reg_jenc_awqos` reader - "]
pub type REG_JENC_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_jenc_awqos` writer - "]
pub type REG_JENC_AWQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_jenc_arqos` reader - "]
pub type REG_JENC_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_jenc_arqos` writer - "]
pub type REG_JENC_ARQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_jdec_awqos` reader - "]
pub type REG_JDEC_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_jdec_awqos` writer - "]
pub type REG_JDEC_AWQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_jdec_arqos` reader - "]
pub type REG_JDEC_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_jdec_arqos` writer - "]
pub type REG_JDEC_ARQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_vdo0_awqos` reader - "]
pub type REG_VDO0_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_vdo0_awqos` writer - "]
pub type REG_VDO0_AWQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_vdo0_arqos` reader - "]
pub type REG_VDO0_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_vdo0_arqos` writer - "]
pub type REG_VDO0_ARQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_vdo1_awqos` reader - "]
pub type REG_VDO1_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_vdo1_awqos` writer - "]
pub type REG_VDO1_AWQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_vdo1_arqos` reader - "]
pub type REG_VDO1_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_vdo1_arqos` writer - "]
pub type REG_VDO1_ARQOS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_ref_awqos` reader - "]
pub type REG_REF_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_ref_awqos` writer - "]
pub type REG_REF_AWQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_ref_arqos` reader - "]
pub type REG_REF_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_ref_arqos` writer - "]
pub type REG_REF_ARQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_cnn_awqos` reader - "]
pub type REG_CNN_AWQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_cnn_awqos` writer - "]
pub type REG_CNN_AWQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reg_cnn_arqos` reader - "]
pub type REG_CNN_ARQOS_R = crate::BitReader<bool>;
#[doc = "Field `reg_cnn_arqos` writer - "]
pub type REG_CNN_ARQOS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CODEC_QOS_CTRL_SPEC, bool, O>;
#[doc = "Field `reserved_12_31` reader - "]
pub type RESERVED_12_31_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_jenc_awqos(&self) -> REG_JENC_AWQOS_R {
        REG_JENC_AWQOS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_jenc_arqos(&self) -> REG_JENC_ARQOS_R {
        REG_JENC_ARQOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reg_jdec_awqos(&self) -> REG_JDEC_AWQOS_R {
        REG_JDEC_AWQOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reg_jdec_arqos(&self) -> REG_JDEC_ARQOS_R {
        REG_JDEC_ARQOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reg_vdo0_awqos(&self) -> REG_VDO0_AWQOS_R {
        REG_VDO0_AWQOS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn reg_vdo0_arqos(&self) -> REG_VDO0_ARQOS_R {
        REG_VDO0_ARQOS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reg_vdo1_awqos(&self) -> REG_VDO1_AWQOS_R {
        REG_VDO1_AWQOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn reg_vdo1_arqos(&self) -> REG_VDO1_ARQOS_R {
        REG_VDO1_ARQOS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn reg_ref_awqos(&self) -> REG_REF_AWQOS_R {
        REG_REF_AWQOS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn reg_ref_arqos(&self) -> REG_REF_ARQOS_R {
        REG_REF_ARQOS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn reg_cnn_awqos(&self) -> REG_CNN_AWQOS_R {
        REG_CNN_AWQOS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn reg_cnn_arqos(&self) -> REG_CNN_ARQOS_R {
        REG_CNN_ARQOS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:31"]
    #[inline(always)]
    pub fn reserved_12_31(&self) -> RESERVED_12_31_R {
        RESERVED_12_31_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn reg_jenc_awqos(&mut self) -> REG_JENC_AWQOS_W<0> {
        REG_JENC_AWQOS_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn reg_jenc_arqos(&mut self) -> REG_JENC_ARQOS_W<1> {
        REG_JENC_ARQOS_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn reg_jdec_awqos(&mut self) -> REG_JDEC_AWQOS_W<2> {
        REG_JDEC_AWQOS_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn reg_jdec_arqos(&mut self) -> REG_JDEC_ARQOS_W<3> {
        REG_JDEC_ARQOS_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vdo0_awqos(&mut self) -> REG_VDO0_AWQOS_W<4> {
        REG_VDO0_AWQOS_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vdo0_arqos(&mut self) -> REG_VDO0_ARQOS_W<5> {
        REG_VDO0_ARQOS_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vdo1_awqos(&mut self) -> REG_VDO1_AWQOS_W<6> {
        REG_VDO1_AWQOS_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn reg_vdo1_arqos(&mut self) -> REG_VDO1_ARQOS_W<7> {
        REG_VDO1_ARQOS_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_awqos(&mut self) -> REG_REF_AWQOS_W<8> {
        REG_REF_AWQOS_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ref_arqos(&mut self) -> REG_REF_ARQOS_W<9> {
        REG_REF_ARQOS_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cnn_awqos(&mut self) -> REG_CNN_AWQOS_W<10> {
        REG_CNN_AWQOS_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn reg_cnn_arqos(&mut self) -> REG_CNN_ARQOS_W<11> {
        REG_CNN_ARQOS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "codec_qos_ctrl\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [codec_qos_ctrl](index.html) module"]
pub struct CODEC_QOS_CTRL_SPEC;
impl crate::RegisterSpec for CODEC_QOS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [codec_qos_ctrl::R](R) reader structure"]
impl crate::Readable for CODEC_QOS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [codec_qos_ctrl::W](W) writer structure"]
impl crate::Writable for CODEC_QOS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets codec_qos_ctrl to value 0"]
impl crate::Resettable for CODEC_QOS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
